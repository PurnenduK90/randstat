/**
 * TestU01 SmallCrush Web UI Dashboard Renderer
 * Renders the 10-test SmallCrush battery evaluation table and metrics.
 */

function renderTestU01Dashboard(container, evalResult, options = {}) {
  if (!container || !evalResult) return;
  const isInitial = options.initial === true;

  const rows = evalResult.entries.map(e => {
    let badge = '';
    if (e.status === 'NOT IMPLEMENTED') {
      badge = '<span style="background:#334155;color:#94a3b8;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:600;">NOT IMPLEMENTED</span>';
    } else if (isInitial) {
      badge = '<span style="background:#0284c722;color:#38bdf8;border:1px solid #38bdf844;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:600;">READY</span>';
    } else if (e.status === 'PASS') {
      badge = '<span style="background:#10b981;color:#fff;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:700;">PASS</span>';
    } else if (e.status === 'FAIL') {
      badge = '<span style="background:#f43f5e;color:#fff;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:700;">FAIL</span>';
    } else if (e.status === 'INSUFFICIENT DATA') {
      badge = '<span style="background:#eab308;color:#1e293b;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:700;">INSUFFICIENT DATA</span>';
    } else {
      badge = '<span style="background:#334155;color:#94a3b8;padding:2px 8px;border-radius:4px;font-size:11px;font-weight:600;">NOT IMPLEMENTED</span>';
    }

    const statStr = isInitial ? '—' : (e.statistic != null ? e.statistic.toFixed(4) : 'N/A');
    const pvalStr = isInitial ? '—' : (e.pValue != null ? e.pValue.toFixed(4) : 'N/A');

    return `
      <tr style="border-bottom:1px solid #1e293b;">
        <td style="padding:10px;color:#94a3b8;font-family:monospace;">${e.id}</td>
        <td style="padding:10px;font-weight:600;color:#f8fafc;">${e.name}</td>
        <td style="padding:10px;color:#cbd5e1;font-size:12px;">${e.battery}</td>
        <td style="padding:10px;color:#38bdf8;font-family:monospace;">${statStr}</td>
        <td style="padding:10px;color:#cbd5e1;font-family:monospace;">${pvalStr}</td>
        <td style="padding:10px;">${badge}</td>
      </tr>
    `;
  }).join('');

  const statusSummary = isInitial
    ? `Active: <strong style="color:#38bdf8;">${evalResult.implementedCount} / ${evalResult.totalTests}</strong> | Status: <strong style="color:#38bdf8;">Ready for test input</strong>`
    : `Active: <strong style="color:#38bdf8;">${evalResult.implementedCount} / ${evalResult.totalTests}</strong> | Passed: <strong style="color:#10b981;">${evalResult.passedCount}</strong> | Failed: <strong style="color:#f43f5e;">${evalResult.failedCount}</strong>`;

  const html = `
    <div class="randstat-testu01-card" style="font-family:system-ui,sans-serif;background:#0f172a;color:#f8fafc;padding:24px;border-radius:12px;border:1px solid #1e293b;">
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px;">
        <h3 style="margin:0;font-size:18px;font-weight:700;color:#38bdf8;">TestU01 PRNG Benchmark (SmallCrush)</h3>
        <div style="font-size:12px;color:#94a3b8;">
          ${statusSummary}
        </div>
      </div>

      <div style="overflow-x:auto;">
        <table style="width:100%;border-collapse:collapse;text-align:left;font-size:13px;">
          <thead>
            <tr style="border-bottom:2px solid #334155;color:#94a3b8;">
              <th style="padding:10px;">ID</th>
              <th style="padding:10px;">Test Name</th>
              <th style="padding:10px;">Battery</th>
              <th style="padding:10px;">Statistic</th>
              <th style="padding:10px;">p-value</th>
              <th style="padding:10px;">Status</th>
            </tr>
          </thead>
          <tbody>
            ${rows}
          </tbody>
        </table>
      </div>
    </div>
  `;

  container.innerHTML = html;
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { renderTestU01Dashboard };
}
