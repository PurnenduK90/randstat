/**
 * ENT Web UI Dashboard Renderer
 * Renders ENT stats cards, verdict badges, and charts.
 */

function renderEntDashboard(container, res, evalResult, options = {}) {
  if (!container || !res || !evalResult) return;

  const verdictBadge = {
    PASS: '<span style="background:#10b981;color:#ffffff;padding:4px 10px;border-radius:6px;font-weight:700;">✓ LIKELY RANDOM</span>',
    WARN: '<span style="background:#f59e0b;color:#1e293b;padding:4px 10px;border-radius:6px;font-weight:700;">⚠ ARTIFICIAL UNIFORMITY</span>',
    FAIL: '<span style="background:#f43f5e;color:#ffffff;padding:4px 10px;border-radius:6px;font-weight:700;">⚠ BIASED / ANOMALY</span>',
  }[evalResult.overallStatus] || evalResult.overallStatus;

  const html = `
    <div class="randstat-ent-card" style="font-family:system-ui,sans-serif;background:#0f172a;color:#f8fafc;padding:24px;border-radius:12px;border:1px solid #1e293b;">
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:20px;">
        <h3 style="margin:0;font-size:18px;font-weight:700;color:#38bdf8;">Fourmilab ENT Randomness Analysis</h3>
        <div>${verdictBadge}</div>
      </div>

      <div style="display:grid;grid-template-columns:repeat(auto-fit, minmax(200px, 1fr));gap:16px;margin-bottom:20px;">
        <div style="background:#1e293b;padding:14px;border-radius:8px;">
          <div style="color:#94a3b8;font-size:12px;">Shannon Entropy</div>
          <div style="font-size:20px;font-weight:700;color:#f8fafc;">${res.entropy.toFixed(6)} <span style="font-size:12px;font-weight:400;color:#94a3b8;">b/B</span></div>
          <div style="font-size:11px;color:#38bdf8;">Compress: ${res.compressionPercent.toFixed(2)}%</div>
        </div>

        <div style="background:#1e293b;padding:14px;border-radius:8px;">
          <div style="color:#94a3b8;font-size:12px;">Chi-Square (df=255)</div>
          <div style="font-size:20px;font-weight:700;color:#f8fafc;">${res.chiSquare.toFixed(2)}</div>
          <div style="font-size:11px;color:#94a3b8;">Exceed: ${(evalResult.pochisqExceedProb * 100).toFixed(2)}%</div>
        </div>

        <div style="background:#1e293b;padding:14px;border-radius:8px;">
          <div style="color:#94a3b8;font-size:12px;">Arithmetic Mean</div>
          <div style="font-size:20px;font-weight:700;color:#f8fafc;">${res.mean.toFixed(4)}</div>
          <div style="font-size:11px;color:#94a3b8;">Ideal: 127.5000</div>
        </div>

        <div style="background:#1e293b;padding:14px;border-radius:8px;">
          <div style="color:#94a3b8;font-size:12px;">Monte Carlo π (2D)</div>
          <div style="font-size:20px;font-weight:700;color:#f8fafc;">${res.monteCarloPi.toFixed(6)}</div>
          <div style="font-size:11px;color:#94a3b8;">Error: ${evalResult.piErrorPercent.toFixed(2)}%</div>
        </div>

        <div style="background:#1e293b;padding:14px;border-radius:8px;">
          <div style="color:#94a3b8;font-size:12px;">Serial Correlation</div>
          <div style="font-size:20px;font-weight:700;color:#f8fafc;">${res.serialCorrelation < -90000 ? 'Undefined' : res.serialCorrelation.toFixed(6)}</div>
          <div style="font-size:11px;color:#94a3b8;">Lag-1 memory</div>
        </div>
      </div>

      <div style="margin-top:10px;font-size:12px;color:#64748b;">
        <span>Stream: <strong>${res.totalBytes.toLocaleString()} bytes</strong></span> | 
        <span>SHA-256: <code style="color:#38bdf8;">${res.sha256}</code></span>
      </div>
    </div>
  `;

  container.innerHTML = html;
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { renderEntDashboard };
}
