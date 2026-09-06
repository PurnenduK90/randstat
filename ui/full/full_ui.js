/**
 * Multi-Suite Meta-Dashboard UI.
 * Renders an aggregated view containing all 8 test batteries in a cohesive tabbed dashboard.
 */
function renderFullDashboard(container, fullData) {
  if (!container || !fullData) return;

  const { totalBytes, sha256, ent, nist, ais31, dieharder, sp80090b, testu01, practrand, gjrand } = fullData;

  const entPassed = ent && ent.evaluation && ent.evaluation.overallStatus === 'PASS';
  const entStatus = ent && ent.evaluation ? ent.evaluation.overallStatus : 'UNKNOWN';

  container.innerHTML = `
    <div class="randstat-full-dashboard" style="font-family: system-ui, -apple-system, sans-serif; display: flex; flex-direction: column; gap: 20px;">
      
      <!-- Top Meta Summary Card -->
      <div style="background: #1e293b; border: 1px solid #334155; border-radius: 10px; padding: 20px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
        <div>
          <div style="display: flex; align-items: center; gap: 10px;">
            <span style="font-size: 1.25rem; font-weight: 700; color: #fff;">Randomness Battery Synthesis</span>
            <span style="font-size: 0.8rem; font-weight: 700; padding: 3px 10px; border-radius: 12px; background: #10b98122; color: #10b981; border: 1px solid #10b981;">
              8 SUITES STREAMED
            </span>
          </div>
          <div style="font-size: 0.82rem; color: #94a3b8; margin-top: 6px; font-family: ui-monospace, monospace;">
            Stream Size: <strong style="color: #f8fafc;">${totalBytes.toLocaleString()} bytes</strong> (${(totalBytes / 1024).toFixed(2)} KB) | SHA-256: <strong style="color: #38bdf8;">${sha256.slice(0, 16)}...${sha256.slice(48)}</strong>
          </div>
        </div>

        <!-- Suite Pass Counters Grid -->
        <div style="display: flex; flex-wrap: wrap; gap: 8px;">
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">ENT</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: ${entPassed ? '#10b981' : '#f59e0b'};">${entStatus}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">NIST 800-22</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${nist.passedCount}/${nist.totalTests}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">AIS 31</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${ais31.passedCount}/${ais31.totalTests}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">Dieharder</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${dieharder.passedCount}/${dieharder.totalTests}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">SP 800-90B</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${sp80090b.passedCount}/${sp80090b.totalTests}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">TestU01</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${testu01.passedCount}/${testu01.totalTests}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">PractRand</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${practrand.passedCount}/${practrand.totalTests}</div>
          </div>
          <div style="background: #0f172a; padding: 6px 10px; border-radius: 6px; border: 1px solid #334155; text-align: center;">
            <div style="font-size: 0.65rem; color: #94a3b8; font-weight: 600;">gjrand</div>
            <div style="font-size: 0.85rem; font-weight: 700; color: #38bdf8;">${gjrand.passedCount}/${gjrand.totalTests}</div>
          </div>
        </div>
      </div>

      <!-- Navigation Tabs for Sub-Dashboards -->
      <div style="display: flex; flex-wrap: wrap; gap: 6px; border-bottom: 1px solid #334155; padding-bottom: 8px;">
        <button id="full-tab-ent" class="full-suite-tab" style="background: #38bdf818; color: #38bdf8; border: 1px solid #38bdf844; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">📊 ENT</button>
        <button id="full-tab-nist" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">🛡️ NIST SP 800-22</button>
        <button id="full-tab-ais31" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">🇩🇪 BSI AIS 31</button>
        <button id="full-tab-dieharder" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">🎲 Dieharder</button>
        <button id="full-tab-sp80090b" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">📐 SP 800-90B</button>
        <button id="full-tab-testu01" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">🧪 TestU01</button>
        <button id="full-tab-practrand" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">⚡ PractRand</button>
        <button id="full-tab-gjrand" class="full-suite-tab" style="background: transparent; color: #94a3b8; border: 1px solid transparent; padding: 6px 12px; border-radius: 6px; font-weight: 600; font-size: 0.8rem; cursor: pointer;">🔬 gjrand</button>
      </div>

      <!-- Sub-Dashboard Containers -->
      <div id="full-panel-ent" class="full-suite-panel" style="display: block;"></div>
      <div id="full-panel-nist" class="full-suite-panel" style="display: none;"></div>
      <div id="full-panel-ais31" class="full-suite-panel" style="display: none;"></div>
      <div id="full-panel-dieharder" class="full-suite-panel" style="display: none;"></div>
      <div id="full-panel-sp80090b" class="full-suite-panel" style="display: none;"></div>
      <div id="full-panel-testu01" class="full-suite-panel" style="display: none;"></div>
      <div id="full-panel-practrand" class="full-suite-panel" style="display: none;"></div>
      <div id="full-panel-gjrand" class="full-suite-panel" style="display: none;"></div>
    </div>
  `;

  // Render individual dashboards into panels
  const entPanel = container.querySelector('#full-panel-ent');
  const nistPanel = container.querySelector('#full-panel-nist');
  const aisPanel = container.querySelector('#full-panel-ais31');
  const dieharderPanel = container.querySelector('#full-panel-dieharder');
  const spPanel = container.querySelector('#full-panel-sp80090b');
  const testu01Panel = container.querySelector('#full-panel-testu01');
  const practrandPanel = container.querySelector('#full-panel-practrand');
  const gjrandPanel = container.querySelector('#full-panel-gjrand');

  if (typeof renderEntDashboard === 'function' && ent) {
    renderEntDashboard(entPanel, ent.result, ent.evaluation);
  }
  if (typeof renderNistDashboard === 'function' && nist) {
    renderNistDashboard(nistPanel, nist);
  }
  if (typeof renderAis31Dashboard === 'function' && ais31) {
    renderAis31Dashboard(aisPanel, ais31);
  }
  if (typeof renderDieharderDashboard === 'function' && dieharder) {
    renderDieharderDashboard(dieharderPanel, dieharder);
  }
  if (typeof renderSp80090bDashboard === 'function' && sp80090b) {
    renderSp80090bDashboard(spPanel, sp80090b);
  }
  if (typeof renderTestU01Dashboard === 'function' && testu01) {
    renderTestU01Dashboard(testu01Panel, testu01);
  }
  if (typeof renderPractRandDashboard === 'function' && practrand) {
    renderPractRandDashboard(practrandPanel, practrand);
  }
  if (typeof renderGjrandDashboard === 'function' && gjrand) {
    renderGjrandDashboard(gjrandPanel, gjrand);
  }

  // Tab switching logic
  const tabs = [
    { btn: container.querySelector('#full-tab-ent'), panel: entPanel },
    { btn: container.querySelector('#full-tab-nist'), panel: nistPanel },
    { btn: container.querySelector('#full-tab-ais31'), panel: aisPanel },
    { btn: container.querySelector('#full-tab-dieharder'), panel: dieharderPanel },
    { btn: container.querySelector('#full-tab-sp80090b'), panel: spPanel },
    { btn: container.querySelector('#full-tab-testu01'), panel: testu01Panel },
    { btn: container.querySelector('#full-tab-practrand'), panel: practrandPanel },
    { btn: container.querySelector('#full-tab-gjrand'), panel: gjrandPanel },
  ];

  tabs.forEach(({ btn, panel }) => {
    if (!btn || !panel) return;
    btn.addEventListener('click', () => {
      tabs.forEach(t => {
        if (t.btn) {
          t.btn.style.background = 'transparent';
          t.btn.style.color = '#94a3b8';
          t.btn.style.borderColor = 'transparent';
        }
        if (t.panel) t.panel.style.display = 'none';
      });
      btn.style.background = '#38bdf818';
      btn.style.color = '#38bdf8';
      btn.style.borderColor = '#38bdf844';
      panel.style.display = 'block';
    });
  });
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    renderFullDashboard,
    FullSuiteRunner: typeof FullSuiteRunner !== 'undefined' ? FullSuiteRunner : undefined,
    renderEntDashboard: typeof renderEntDashboard !== 'undefined' ? renderEntDashboard : undefined,
    renderNistDashboard: typeof renderNistDashboard !== 'undefined' ? renderNistDashboard : undefined,
    renderAis31Dashboard: typeof renderAis31Dashboard !== 'undefined' ? renderAis31Dashboard : undefined,
    renderDieharderDashboard: typeof renderDieharderDashboard !== 'undefined' ? renderDieharderDashboard : undefined,
    renderSp80090bDashboard: typeof renderSp80090bDashboard !== 'undefined' ? renderSp80090bDashboard : undefined,
    renderTestU01Dashboard: typeof renderTestU01Dashboard !== 'undefined' ? renderTestU01Dashboard : undefined,
    renderPractRandDashboard: typeof renderPractRandDashboard !== 'undefined' ? renderPractRandDashboard : undefined,
    renderGjrandDashboard: typeof renderGjrandDashboard !== 'undefined' ? renderGjrandDashboard : undefined,
  };
}
