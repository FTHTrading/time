// ==========================================================================
// ALL COUCH NO CAGE — CRYPTOGRAPHIC RECEIPTS & EVIDENCE HASHING
// ==========================================================================

(function (window) {
  'use strict';

  async function sha256(data) {
    const text = typeof data === 'string' ? data : JSON.stringify(data);
    if (!window.crypto || !window.crypto.subtle) {
      let hash = 0;
      for (let i = 0; i < text.length; i++) {
        hash = (hash << 5) - hash + text.charCodeAt(i);
        hash |= 0;
      }
      return 'sha256:sim_' + Math.abs(hash).toString(16).padStart(16, '0');
    }
    const encoder = new TextEncoder();
    const dataBuffer = encoder.encode(text);
    const hashBuffer = await window.crypto.subtle.digest('SHA-256', dataBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return 'sha256:' + hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  }

  function downloadFile(filename, content, mimeType = 'application/json') {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  }

  window.ACNC_Receipts = {
    sha256,
    downloadFile
  };
})(window);
