// ==========================================================================
// ALL COUCH NO CAGE — CANONICAL STATE & POLICY ENGINE (V4.0)
// ==========================================================================

(function (window) {
  'use strict';

  const STORAGE_KEYS = {
    ACTIVITIES: 'acnc_activities_v4',
    REDUCTIONS: 'acnc_reductions_v4',
    OFFSETS: 'acnc_offsets_v4',
    FOCUS: 'acnc_focus_v4',
    TIMER: 'acnc_timer_v4',
    WALLET: 'acnc_wallet_v4'
  };

  const DATA_STATUS = {
    USER_ENTERED: { label: 'USER_ENTERED', mult: 0.2, badgeClass: 'badge-local' },
    RECEIPT_BACKED: { label: 'RECEIPT_BACKED', mult: 0.8, badgeClass: 'badge-local' },
    METERED: { label: 'METERED', mult: 1.0, badgeClass: 'badge-live' },
    ATTESTED: { label: 'ATTESTED', mult: 1.0, badgeClass: 'badge-live' },
    REGISTRY_VERIFIED: { label: 'REGISTRY_VERIFIED', mult: 1.0, badgeClass: 'badge-verified' },
    ESTIMATED: { label: 'ESTIMATED', mult: 0.3, badgeClass: 'badge-nodata' },
    UNVERIFIED: { label: 'UNVERIFIED', mult: 0.0, badgeClass: 'badge-rejected' }
  };

  const FACTORS = {
    version: '2026.1',
    electricity_kwh: { factor: 0.385, unit: 'kWh', name: 'Home Electricity (US Avg)', source: 'EPA eGRID 2024' },
    gasoline_car_mile: { factor: 0.404, unit: 'miles', name: 'Gasoline Vehicle Travel', source: 'EPA GHG Factors' },
    transit_bus_mile: { factor: 0.140, unit: 'miles', name: 'Public Transit Bus/Rail', source: 'DOT FTA' },
    food_waste_kg: { factor: 2.500, unit: 'kg', name: 'Food Waste to Landfill', source: 'EPA WARM' },
    cloud_gpu_hour: { factor: 0.180, unit: 'hours', name: 'Cloud GPU Compute', source: 'Sustainability Disclosures' }
  };

  function getList(key) {
    try {
      const d = localStorage.getItem(key);
      return d ? JSON.parse(d) : [];
    } catch (e) {
      return [];
    }
  }

  function setList(key, list) {
    try {
      localStorage.setItem(key, JSON.stringify(list));
    } catch (e) {}
  }

  const State = {
    DATA_STATUS,
    FACTORS,
    getActivities() { return getList(STORAGE_KEYS.ACTIVITIES); },
    addActivity(rec) {
      const list = getList(STORAGE_KEYS.ACTIVITIES);
      list.unshift(rec);
      setList(STORAGE_KEYS.ACTIVITIES, list);
      return rec;
    },
    getReductions() { return getList(STORAGE_KEYS.REDUCTIONS); },
    addReduction(rec) {
      const list = getList(STORAGE_KEYS.REDUCTIONS);
      list.unshift(rec);
      setList(STORAGE_KEYS.REDUCTIONS, list);
      return rec;
    },
    getOffsets() { return getList(STORAGE_KEYS.OFFSETS); },
    addOffset(rec) {
      const list = getList(STORAGE_KEYS.OFFSETS);
      list.unshift(rec);
      setList(STORAGE_KEYS.OFFSETS, list);
      return rec;
    },
    getFocusSessions() { return getList(STORAGE_KEYS.FOCUS); },
    addFocusSession(rec) {
      const list = getList(STORAGE_KEYS.FOCUS);
      list.unshift(rec);
      setList(STORAGE_KEYS.FOCUS, list);
      return rec;
    },
    getSummary() {
      const acts = this.getActivities();
      const reds = this.getReductions();
      const offs = this.getOffsets();
      const focs = this.getFocusSessions();

      let focusPts = 0;
      let impactPts = 0;
      let contributionPts = 0;
      let totalVTime = 0;
      let todayPts = 0;

      const todayStr = new Date().toISOString().split('T')[0];

      focs.forEach(f => {
        const p = f.pointsEarned || 0;
        focusPts += p;
        totalVTime += p * 0.10 * 0.8;
        if (f.timestamp && f.timestamp.startsWith(todayStr)) todayPts += p;
      });

      reds.forEach(r => {
        const p = r.pointsEarned || 0;
        impactPts += p;
        totalVTime += p * 0.10 * 0.8;
        if (r.timestamp && r.timestamp.startsWith(todayStr)) todayPts += p;
      });

      offs.forEach(o => {
        const p = o.pointsEarned || 0;
        contributionPts += p;
        totalVTime += p * 0.10 * 1.0;
        if (o.timestamp && o.timestamp.startsWith(todayStr)) todayPts += p;
      });

      const totalPts = focusPts + impactPts + contributionPts;
      return {
        hasData: (acts.length + reds.length + offs.length + focs.length) > 0,
        focusPts,
        impactPts,
        contributionPts,
        totalPts,
        todayPts,
        eligibleVTime: parseFloat(Math.min(totalVTime, 200).toFixed(2)),
        focusStreak: focs.length > 0 ? `${focs.length} Sprints` : 'Not started'
      };
    },
    exportJson() {
      return JSON.stringify({
        protocol: 'ALL COUCH NO CAGE',
        version: '4.0.0',
        exportedAt: new Date().toISOString(),
        activities: this.getActivities(),
        reductions: this.getReductions(),
        offsets: this.getOffsets(),
        focusSessions: this.getFocusSessions(),
        summary: this.getSummary()
      }, null, 2);
    }
  };

  window.ACNC_State = State;
})(window);
