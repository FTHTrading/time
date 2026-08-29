// ==========================================================================
// ALL COUCH NO CAGE — WEB3 WALLET CONNECTOR (DISCONNECTED DEFAULT)
// ==========================================================================

(function (window) {
  'use strict';

  const STORAGE_KEY = 'acnc_wallet_v4';

  const Wallet = {
    getState() {
      try {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) return JSON.parse(stored);
      } catch (e) {}
      return { isConnected: false, address: null, network: 'Polygon Amoy' };
    },

    async connect() {
      if (window.ethereum) {
        try {
          const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
          const chainId = await window.ethereum.request({ method: 'eth_chainId' });
          const state = {
            isConnected: true,
            address: accounts[0],
            network: chainId === '0x13882' ? 'Polygon Amoy (80002)' : 'EVM Network (' + chainId + ')'
          };
          localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
          return state;
        } catch (e) {}
      }
      const randomAddr = '0x' + Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join('');
      const state = {
        isConnected: true,
        address: randomAddr,
        network: 'Polygon Amoy (80002)'
      };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
      return state;
    },

    disconnect() {
      localStorage.removeItem(STORAGE_KEY);
      return { isConnected: false, address: null, network: 'Polygon Amoy' };
    }
  };

  window.ACNC_Wallet = Wallet;
})(window);
