// ==========================================================================
// ALL COUCH NO CAGE — MASTER MULTI-PAGE CONTROLLER (V4.0)
// ==========================================================================

document.addEventListener('DOMContentLoaded', () => {
  initGlobalHeader();

  const page = document.body.dataset.page;
  switch (page) {
    case 'overview':
      initOverviewPage();
      break;
    case 'measure':
      initMeasurePage();
      break;
    case 'reduce':
      initReducePage();
      break;
    case 'offsets':
      initOffsetsPage();
      break;
    case 'rewards':
      initRewardsPage();
      break;
    case 'vault':
      initVaultPage();
      break;
    case 'relics':
      initRelicsPage();
      break;
    case 'protocol':
      initProtocolPage();
      break;
  }
});

function initGlobalHeader() {
  const btn = document.getElementById('globalWalletBtn');
  if (!btn || !window.ACNC_Wallet) return;

  function update() {
    const w = window.ACNC_Wallet.getState();
    if (w.isConnected && w.address) {
      btn.textContent = w.address.substring(0, 6) + '...' + w.address.substring(w.address.length - 4);
      btn.className = 'btn btn-secondary btn-sm';
    } else {
      btn.textContent = 'Connect Wallet';
      btn.className = 'btn btn-primary btn-sm';
    }
  }

  btn.addEventListener('click', async () => {
    const w = window.ACNC_Wallet.getState();
    if (!w.isConnected) {
      await window.ACNC_Wallet.connect();
      update();
      if (document.body.dataset.page === 'vault') initVaultPage();
    } else {
      if (document.body.dataset.page !== 'vault') window.location.href = 'vault.html';
    }
  });

  update();
}

// 1. OVERVIEW PAGE
function initOverviewPage() {
  const summary = window.ACNC_State.getSummary();
  const emptyBanner = document.getElementById('overviewEmptyBanner');
  const summaryGrid = document.getElementById('overviewSummaryGrid');

  if (emptyBanner && summaryGrid) {
    if (summary.hasData) {
      emptyBanner.style.display = 'none';
      summaryGrid.style.display = 'grid';
    } else {
      emptyBanner.style.display = 'block';
      summaryGrid.style.display = 'none';
    }
  }

  const elFoc = document.getElementById('overviewFocusPts');
  const elImp = document.getElementById('overviewImpactPts');
  const elOff = document.getElementById('overviewOffsetPts');
  const elVTime = document.getElementById('overviewVTime');

  if (elFoc) elFoc.textContent = summary.focusPts;
  if (elImp) elImp.textContent = summary.impactPts;
  if (elOff) elOff.textContent = summary.contributionPts;
  if (elVTime) elVTime.textContent = `${summary.eligibleVTime.toFixed(2)} VTIME`;
}

// 2. MEASURE PAGE
function initMeasurePage() {
  const domainSelect = document.getElementById('measureDomainSelect');
  const qtyInput = document.getElementById('measureQtyInput');
  const proofSelect = document.getElementById('measureProofSelect');
  const co2El = document.getElementById('measureCalculatedCo2');
  const addBtn = document.getElementById('measureAddBtn');
  const tableBody = document.getElementById('measureTableBody');
  const emptyNotice = document.getElementById('measureEmptyNotice');

  function calc() {
    if (!domainSelect || !qtyInput || !co2El) return;
    const d = domainSelect.value;
    const q = Number(qtyInput.value) || 0;
    const factorObj = window.ACNC_State.FACTORS[d];
    if (factorObj) {
      co2El.textContent = `${(q * factorObj.factor).toFixed(2)} kg CO2e`;
    }
  }

  if (domainSelect) domainSelect.addEventListener('change', calc);
  if (qtyInput) qtyInput.addEventListener('input', calc);
  calc();

  if (addBtn) {
    addBtn.addEventListener('click', async () => {
      const d = domainSelect.value;
      const q = Number(qtyInput.value) || 0;
      const status = proofSelect.value;
      const factorObj = window.ACNC_State.FACTORS[d];
      const co2 = factorObj ? q * factorObj.factor : 0;
      const id = 'rec_' + Math.random().toString(36).substring(2, 10);
      const hash = await window.ACNC_Receipts.sha256({ id, d, q, status, t: Date.now() });

      window.ACNC_State.addActivity({
        recordId: id,
        category: d,
        quantity: q,
        unit: factorObj ? factorObj.unit : '',
        dataStatus: status,
        evidenceHash: hash,
        factorSource: factorObj ? factorObj.source : 'Published standard',
        factorVersion: '2026.1',
        co2eKgEstimate: parseFloat(co2.toFixed(2)),
        timestamp: new Date().toISOString()
      });

      render();
      alert('Activity record saved to local client storage.');
    });
  }

  function render() {
    if (!tableBody) return;
    const list = window.ACNC_State.getActivities();
    if (list.length === 0) {
      tableBody.innerHTML = '';
      if (emptyNotice) emptyNotice.style.display = 'block';
    } else {
      if (emptyNotice) emptyNotice.style.display = 'none';
      tableBody.innerHTML = list.map(item => {
        const factorObj = window.ACNC_State.FACTORS[item.category];
        const name = factorObj ? factorObj.name : item.category;
        const statusMeta = window.ACNC_State.DATA_STATUS[item.dataStatus] || window.ACNC_State.DATA_STATUS.RECEIPT_BACKED;
        return `
          <tr>
            <td><strong>${name}</strong></td>
            <td>${item.quantity} ${item.unit}</td>
            <td><strong>${item.co2eKgEstimate} kg</strong></td>
            <td><span class="badge ${statusMeta.badgeClass}">${item.dataStatus}</span></td>
            <td><span class="evidence-hash"><i class="fa-solid fa-fingerprint"></i> ${item.evidenceHash.substring(0, 16)}...</span></td>
          </tr>
        `;
      }).join('');
    }
  }

  render();
}

// 3. REDUCE PAGE
function initReducePage() {
  const typeSelect = document.getElementById('reduceTypeSelect');
  const qtyInput = document.getElementById('reduceQtyInput');
  const avoidedCo2El = document.getElementById('reduceAvoidedCo2');
  const earnedPtsEl = document.getElementById('reduceEarnedPts');
  const addBtn = document.getElementById('reduceAddBtn');
  const tableBody = document.getElementById('reduceTableBody');
  const emptyNotice = document.getElementById('reduceEmptyNotice');

  function calc() {
    if (!typeSelect || !qtyInput || !avoidedCo2El) return;
    const t = typeSelect.value;
    const q = Number(qtyInput.value) || 0;
    let avoided = 0;
    if (t === 'electricity') avoided = q * 0.385;
    else if (t === 'transit') avoided = q * 0.404;
    else if (t === 'repair') avoided = q * 4.50;

    const pts = Math.round(avoided * 2.5);
    avoidedCo2El.textContent = `${avoided.toFixed(2)} kg CO2e`;
    if (earnedPtsEl) earnedPtsEl.textContent = `+${pts} Impact Points`;
  }

  if (typeSelect) typeSelect.addEventListener('change', calc);
  if (qtyInput) qtyInput.addEventListener('input', calc);
  calc();

  if (addBtn) {
    addBtn.addEventListener('click', async () => {
      const t = typeSelect.value;
      const q = Number(qtyInput.value) || 0;
      let avoided = 0;
      if (t === 'electricity') avoided = q * 0.385;
      else if (t === 'transit') avoided = q * 0.404;
      else if (t === 'repair') avoided = q * 4.50;

      const pts = Math.round(avoided * 2.5);
      const id = 'red_' + Math.random().toString(36).substring(2, 10);
      const hash = await window.ACNC_Receipts.sha256({ id, t, q, avoided, time: Date.now() });

      window.ACNC_State.addReduction({
        recordId: id,
        type: t,
        quantity: q,
        co2eReducedKg: parseFloat(avoided.toFixed(2)),
        pointsEarned: pts,
        dataStatus: 'RECEIPT_BACKED',
        evidenceHash: hash,
        timestamp: new Date().toISOString()
      });

      render();
      alert('Reduction record saved.');
    });
  }

  function render() {
    if (!tableBody) return;
    const list = window.ACNC_State.getReductions();
    if (list.length === 0) {
      tableBody.innerHTML = '';
      if (emptyNotice) emptyNotice.style.display = 'block';
    } else {
      if (emptyNotice) emptyNotice.style.display = 'none';
      tableBody.innerHTML = list.map(item => `
        <tr>
          <td><strong>${item.type.toUpperCase()}</strong></td>
          <td>${item.quantity}</td>
          <td style="color: var(--status-live);">-${item.co2eReducedKg} kg (Avoided)</td>
          <td><strong style="color: var(--status-verified);">+${item.pointsEarned} Pts</strong></td>
          <td><span class="badge badge-local">${item.dataStatus}</span></td>
        </tr>
      `).join('');
    }
  }

  render();
}

// 4. OFFSETS PAGE
function initOffsetsPage() {
  const regSelect = document.getElementById('offsetRegistrySelect');
  const serialInput = document.getElementById('offsetSerialInput');
  const tonnesInput = document.getElementById('offsetTonnesInput');
  const addBtn = document.getElementById('offsetAddBtn');
  const tableBody = document.getElementById('offsetTableBody');
  const emptyNotice = document.getElementById('offsetEmptyNotice');

  if (addBtn) {
    addBtn.addEventListener('click', async () => {
      const reg = regSelect.value;
      const serial = serialInput.value.trim();
      const tonnes = Number(tonnesInput.value) || 1.0;

      if (!serial) {
        alert('Please enter a valid certificate serial number.');
        return;
      }

      const id = 'ret_' + Math.random().toString(36).substring(2, 10);
      const hash = await window.ACNC_Receipts.sha256({ id, reg, serial, tonnes, time: Date.now() });

      window.ACNC_State.addOffset({
        recordId: id,
        registry: reg,
        serialNumber: serial,
        tonnesCo2eRetired: tonnes,
        pointsEarned: Math.round(tonnes * 100),
        validationStatus: 'REGISTRY_VERIFIED',
        evidenceHash: hash,
        timestamp: new Date().toISOString()
      });

      render();
      serialInput.value = '';
      alert('Offset certificate validated and recorded.');
    });
  }

  function render() {
    if (!tableBody) return;
    const list = window.ACNC_State.getOffsets();
    if (list.length === 0) {
      tableBody.innerHTML = '';
      if (emptyNotice) emptyNotice.style.display = 'block';
    } else {
      if (emptyNotice) emptyNotice.style.display = 'none';
      tableBody.innerHTML = list.map(item => `
        <tr>
          <td><strong>${item.registry}</strong></td>
          <td><code>${item.serialNumber}</code></td>
          <td><strong>${item.tonnesCo2eRetired} Tonnes</strong></td>
          <td><span class="badge badge-verified">${item.validationStatus}</span></td>
          <td><strong style="color: var(--status-verified);">+${item.pointsEarned} Pts</strong></td>
        </tr>
      `).join('');
    }
  }

  render();
}

// 5. REWARDS PAGE
function initRewardsPage() {
  const s = window.ACNC_State.getSummary();
  const elToday = document.getElementById('rewardsTodayPts');
  const elTotal = document.getElementById('rewardsTotalPts');
  const elVTime = document.getElementById('rewardsVTime');
  const elStreak = document.getElementById('rewardsStreak');

  if (elToday) elToday.textContent = s.todayPts;
  if (elTotal) elTotal.textContent = s.totalPts;
  if (elVTime) elVTime.textContent = `${s.eligibleVTime.toFixed(2)} VTIME`;
  if (elStreak) elStreak.textContent = s.focusStreak;
}

// 6. VAULT PAGE
function initVaultPage() {
  const wallet = window.ACNC_Wallet.getState();
  const summary = window.ACNC_State.getSummary();

  const addrEl = document.getElementById('vaultWalletAddr');
  const statusEl = document.getElementById('vaultWalletStatus');
  const vtimeEl = document.getElementById('vaultEligibleVTime');
  const connectBtn = document.getElementById('vaultConnectBtn');
  const disconnectBtn = document.getElementById('vaultDisconnectBtn');
  const exportBtn = document.getElementById('vaultExportBtn');

  if (wallet.isConnected && wallet.address) {
    if (addrEl) addrEl.textContent = wallet.address;
    if (statusEl) statusEl.innerHTML = `<span class="badge badge-live">CONNECTED (${wallet.network})</span>`;
    if (connectBtn) connectBtn.style.display = 'none';
    if (disconnectBtn) disconnectBtn.style.display = 'inline-flex';
  } else {
    if (addrEl) addrEl.textContent = 'Not Connected';
    if (statusEl) statusEl.innerHTML = `<span class="badge badge-nodata">NO WALLET CONNECTED</span>`;
    if (connectBtn) connectBtn.style.display = 'inline-flex';
    if (disconnectBtn) disconnectBtn.style.display = 'none';
  }

  if (vtimeEl) vtimeEl.textContent = `${summary.eligibleVTime.toFixed(2)} VTIME`;

  if (connectBtn) {
    connectBtn.addEventListener('click', async () => {
      await window.ACNC_Wallet.connect();
      initVaultPage();
      initGlobalHeader();
    });
  }

  if (disconnectBtn) {
    disconnectBtn.addEventListener('click', () => {
      window.ACNC_Wallet.disconnect();
      initVaultPage();
      initGlobalHeader();
    });
  }

  if (exportBtn) {
    exportBtn.addEventListener('click', () => {
      const json = window.ACNC_State.exportJson();
      window.ACNC_Receipts.downloadFile(`acnc_ledger_${new Date().toISOString().split('T')[0]}.json`, json);
    });
  }
}

// 7. RELICS PAGE
function initRelicsPage() {
  const summary = window.ACNC_State.getSummary();
  const emptyNotice = document.getElementById('relicsEmptyNotice');
  const grid = document.getElementById('relicsGrid');

  if (emptyNotice && grid) {
    if (summary.hasData) {
      emptyNotice.style.display = 'none';
      grid.style.display = 'grid';
    } else {
      emptyNotice.style.display = 'block';
      grid.style.display = 'none';
    }
  }
}

// 8. PROTOCOL PAGE
function initProtocolPage() {
  const tabs = document.querySelectorAll('.tab-link');
  const panes = document.querySelectorAll('.tab-content');

  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      tabs.forEach(t => t.classList.remove('active'));
      panes.forEach(p => p.classList.remove('active'));

      tab.classList.add('active');
      const target = document.getElementById(tab.dataset.target);
      if (target) target.classList.add('active');
    });
  });
}
