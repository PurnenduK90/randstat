/**
 * Multi-Suite Meta-Dashboard UI.
 * Renders an aggregated view containing ENT, NIST SP 800-22, and BSI AIS 31 results together.
 */
function renderFullDashboard(container, fullData) {
  if (!container || !fullData) return;

  const { totalBytes, sha256, ent, nist, ais31 } = fullData;

  const entPassed = ent.evaluation.overallStatus === 'PASS';
  const nistPassed = nist.overallPassed;
  const aisPassed = ais31.overallPassed;

  const allPassed = entPassed && nistPassed && aisPassed;
  const overallColor = allPassed ? '#10b981' : (ent.evaluation.overallStatus === 'WARN' ? '#f59e0b' : '#ef4444');
  const overallBadgeText = allPassed ? 'PASSED ALL SUITES' : (ent.evaluation.overallStatus === 'WARN' ? 'SUSPICIOUS / WARN' : 'FAILED GUARDRAILS');

  container.innerHTML = `
    <div class="randstat-full-dashboard" style="font-family: system-ui, -apple-system, sans-serif; display: flex; flex-direction: column; gap: 20px;">
      
      <!-- Top Meta Summary Card -->
      <div style="background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 20px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
        <div>
          <div style="display: flex; align-items: center; gap: 10px;">
            <span style="font-size: 1.25rem; font-weight: 700; color: #fff;">Randomness Battery Synthesis</span>
            <span style="font-size: 0.8rem; font-weight: 700; padding: 3px 10px; border-radius: 12px; background: ${overallColor}22; color: ${overallColor}; border: 1px solid ${overallColor};">
              ${overallBadgeText}
            </span>
          </div>
          <div style="font-size: 0.82rem; color: #94a3b8; margin-top: 6px; font-family: ui-monospace, monospace;">
            Stream Size: <strong style="color: #f8fafc;">${totalBytes.toLocaleString()} bytes</strong> (${(totalBytes / 1024).toFixed(2)} KB) | SHA-256: <strong style="color: #38bdf8;">${sha256.slice(0, 16)}...${sha256.slice(48)}</strong>
          </div>
        </div>

        <!-- Suite Pass Counters -->
        <div style="display: flex; gap: 12px;">
          <div style="background: #0f172a; padding: 8px 14px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.7rem; color: #94a3b8; font-weight: 600;">ENT Suite</div>
            <div style="font-size: 0.95rem; font-weight: 700; color: ${entPassed ? '#10b981' : '#f59e0b'};">${ent.evaluation.overallStatus}</div>
          </div>
          <div style="background: #0f172a; padding: 8px 14px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.7rem; color: #94a3b8; font-weight: 600;">NIST SP 800-22</div>
            <div style="font-size: 0.95rem; font-weight: 700; color: ${nistPassed ? '#10b981' : '#ef4444'};">${nist.passedCount}/${nist.totalTests} Passed</div>
          </div>
          <div style="background: #0f172a; padding: 8px 14px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.7rem; color: #94a3b8; font-weight: 600;">BSI AIS 31</div>
            <div style="font-size: 0.95rem; font-weight: 700; color: ${aisPassed ? '#10b981' : '#ef4444'};">${ais31.passedCount}/${ais31.totalTests} Passed</div>
          </div>
        </div>
      </div>

      <!-- Navigation Tabs for Sub-Dashboards -->
      <div style="display: flex; gap: 8px; border-bottom: 1px solid #334155; padding-bottom: 8px;">
        <button id="full-tab-ent" style="background: #38bdf818; color: #38bdf8; border: 1px solid #38bdf844; padding: 6px 14px; border-radius: 6px; font-weight: 600; font-size: 0.85rem; cursor: pointer;">📊 Fourmilab ENT (${ent.result.entropy.toFixed(4)} b/B)</button>
        <button id="full-tab-nist" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 14px; border-radius: 6px; font-weight: 600; font-size: 0.85rem; cursor: pointer;">🛡️ NIST SP 800-22 (${nist.passedCount}/${nist.totalTests})</button>
        <button id="full-tab-ais31" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 14px; border-radius: 6px; font-weight: 600; font-size: 0.85rem; cursor: pointer;">🇩🇪 BSI AIS 31 (${ais31.passedCount}/${ais31.totalTests})</button>
      </div>

      <!-- Sub-Dashboard Containers -->
      <div id="full-panel-ent" style="display: block;"></div>
      <div id="full-panel-nist" style="display: none;"></div>
      <div id="full-panel-ais31" style="display: none;"></div>
    </div>
  `;

  // Render individual dashboards into panels
  const entPanel = container.querySelector('#full-panel-ent');
  const nistPanel = container.querySelector('#full-panel-nist');
  const aisPanel = container.querySelector('#full-panel-ais31');

  if (typeof renderEntDashboard === 'function') {
    renderEntDashboard(entPanel, ent.result, ent.evaluation);
  }
  if (typeof renderNistDashboard === 'function') {
    renderNistDashboard(nistPanel, nist);
  }
  if (typeof renderAis31Dashboard === 'function') {
    renderAis31Dashboard(aisPanel, ais31);
  }

  // Tab switching logic
  const tabEnt = container.querySelector('#full-tab-ent');
  const tabNist = container.querySelector('#full-tab-nist');
  const tabAis = container.querySelector('#full-tab-ais31');

  function setTab(activeTab, activePanel) {
    [tabEnt, tabNist, tabAis].forEach(t => {
      t.style.background = 'transparent';
      t.style.color = '#94a3b8';
      t.style.borderColor = 'transparent';
    });
    [entPanel, nistPanel, aisPanel].forEach(p => p.style.display = 'none');

    activeTab.style.background = '#38bdf818';
    activeTab.style.color = '#38bdf8';
    activeTab.style.borderColor = '#38bdf844';
    activePanel.style.display = 'block';
  }

  tabEnt.addEventListener('click', () => setTab(tabEnt, entPanel));
  tabNist.addEventListener('click', () => setTab(tabNist, nistPanel));
  tabAis.addEventListener('click', () => setTab(tabAis, aisPanel));
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { renderFullDashboard };
}
