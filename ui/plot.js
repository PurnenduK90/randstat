/**
 * HTML5 Canvas Chi-Square & Normal PDF Plot Renderer
 * Renders smooth probability density curves with two-tailed cutoffs and decision regions.
 */

function getChi2Points(wasmInstance, df, xMin, xMax, steps = 300) {
  const func = wasmInstance && (wasmInstance.exports.generate_chi2_points || wasmInstance.exports.wasm_generate_chi2_points);
  if (!wasmInstance || !func) {
    return [];
  }

  const floatCount = func(df, xMin, xMax, steps);
  const ptr = wasmInstance.exports.get_buffer_ptr();
  const f64Array = new Float64Array(wasmInstance.exports.memory.buffer, ptr, floatCount);

  const points = [];
  const pointCount = floatCount / 2;
  for (let i = 0; i < pointCount; i++) {
    points.push({ x: f64Array[i * 2], y: f64Array[i * 2 + 1] });
  }
  return points;
}

function getNormalPoints(wasmInstance, df, xMin, xMax, steps = 300) {
  const func = wasmInstance && (wasmInstance.exports.generate_normal_points || wasmInstance.exports.wasm_generate_normal_points);
  if (!wasmInstance || !func) {
    return [];
  }

  const floatCount = func(df, xMin, xMax, steps);
  const ptr = wasmInstance.exports.get_buffer_ptr();
  const f64Array = new Float64Array(wasmInstance.exports.memory.buffer, ptr, floatCount);

  const points = [];
  const pointCount = floatCount / 2;
  for (let i = 0; i < pointCount; i++) {
    points.push({ x: f64Array[i * 2], y: f64Array[i * 2 + 1] });
  }
  return points;
}

function renderDistributionPlot(canvas, chi2Points, normalPoints, options = {}) {
  if (!canvas || !chi2Points || chi2Points.length === 0) return;
  const ctx = canvas.getContext('2d');
  const dpr = window.devicePixelRatio || 1;

  // Handle high-DPI scaling
  const displayWidth = canvas.clientWidth || 800;
  const displayHeight = canvas.clientHeight || 300;
  if (canvas.width !== displayWidth * dpr || canvas.height !== displayHeight * dpr) {
    canvas.width = displayWidth * dpr;
    canvas.height = displayHeight * dpr;
  }
  ctx.save();
  ctx.scale(dpr, dpr);

  const padding = { top: 35, right: 35, bottom: 45, left: 55 };
  const width = displayWidth - padding.left - padding.right;
  const height = displayHeight - padding.top - padding.bottom;

  ctx.clearRect(0, 0, displayWidth, displayHeight);

  // Compute X & Y bounds
  const xValues = chi2Points.map(p => p.x);
  const xMin = Math.min(...xValues);
  const xMax = Math.max(...xValues);

  const yMaxChi2 = Math.max(...chi2Points.map(p => p.y));
  const yMaxNorm = normalPoints && normalPoints.length > 0 ? Math.max(...normalPoints.map(p => p.y)) : 0;
  const yMax = Math.max(yMaxChi2, yMaxNorm) * 1.15 || 0.02;

  const toScreenX = (x) => padding.left + ((x - xMin) / (xMax - xMin)) * width;
  const toScreenY = (y) => padding.top + height - (y / yMax) * height;

  const lowerCutoff = options.lowerCutoff != null ? options.lowerCutoff : 218.42;
  const upperCutoff = options.upperCutoff != null ? options.upperCutoff : 293.25;

  // --- 1. Draw 3 Decision Regions (Matching Verdict Colors & Naming) ---
  const screenLower = toScreenX(lowerCutoff);
  const screenUpper = toScreenX(upperCutoff);
  const screenLeft = padding.left;
  const screenRight = padding.left + width;

  // Region A: TOO UNIFORM (x < lowerCutoff) - Amber
  ctx.fillStyle = 'rgba(245, 158, 11, 0.10)';
  ctx.fillRect(screenLeft, padding.top, Math.max(0, screenLower - screenLeft), height);

  // Region B: LIKELY RANDOM (lowerCutoff <= x <= upperCutoff) - Emerald Green
  ctx.fillStyle = 'rgba(16, 185, 129, 0.10)';
  ctx.fillRect(screenLower, padding.top, Math.max(0, screenUpper - screenLower), height);

  // Region C: NON-UNIFORM (x > upperCutoff) - Rose Red
  ctx.fillStyle = 'rgba(244, 63, 94, 0.10)';
  ctx.fillRect(screenUpper, padding.top, Math.max(0, screenRight - screenUpper), height);

  // Decision Region Labels at Top
  ctx.font = '600 11px system-ui, sans-serif';
  ctx.textAlign = 'center';

  // Left Label
  if (screenLower - screenLeft > 40) {
    ctx.fillStyle = '#f59e0b';
    ctx.fillText('⚠ ARTIFICIAL UNIFORMITY/STRUCTURED', (screenLeft + screenLower) / 2, padding.top + 16);
  }

  // Center Label
  ctx.fillStyle = '#10b981';
  ctx.fillText('✓ LIKELY RANDOM', (screenLower + screenUpper) / 2, padding.top + 16);

  // Right Label
  if (screenRight - screenUpper > 40) {
    ctx.fillStyle = '#f43f5e';
    ctx.fillText('⚠ LIKELY BIASED/ANOMALY', (screenUpper + screenRight) / 2, padding.top + 16);
  }

  // --- 2. Draw Grid & Axes ---
  ctx.strokeStyle = '#1e293b';
  ctx.lineWidth = 1;
  ctx.beginPath();
  const yTicks = 4;
  for (let i = 0; i <= yTicks; i++) {
    const yVal = (yMax / yTicks) * i;
    const sy = toScreenY(yVal);
    ctx.moveTo(padding.left, sy);
    ctx.lineTo(padding.left + width, sy);

    ctx.fillStyle = '#64748b';
    ctx.font = '10px system-ui, sans-serif';
    ctx.textAlign = 'right';
    ctx.fillText(yVal.toFixed(4), padding.left - 8, sy + 3);
  }
  ctx.stroke();

  // X Axis Ticks
  const xTicks = 6;
  ctx.textAlign = 'center';
  for (let i = 0; i <= xTicks; i++) {
    const xVal = xMin + ((xMax - xMin) / xTicks) * i;
    const sx = toScreenX(xVal);
    ctx.fillStyle = '#64748b';
    ctx.font = '10px system-ui, sans-serif';
    ctx.fillText(xVal.toFixed(1), sx, padding.top + height + 18);
  }

  // --- 3. Draw Normal Distribution Curve ---
  if (normalPoints && normalPoints.length > 0) {
    ctx.beginPath();
    ctx.strokeStyle = 'rgba(148, 163, 184, 0.4)';
    ctx.setLineDash([4, 4]);
    ctx.lineWidth = 1.5;
    for (let i = 0; i < normalPoints.length; i++) {
      const sx = toScreenX(normalPoints[i].x);
      const sy = toScreenY(normalPoints[i].y);
      if (i === 0) ctx.moveTo(sx, sy);
      else ctx.lineTo(sx, sy);
    }
    ctx.stroke();
    ctx.setLineDash([]);
  }

  // --- 4. Draw Chi-Square Distribution Curve ---
  ctx.beginPath();
  ctx.strokeStyle = '#38bdf8';
  ctx.lineWidth = 2.5;
  for (let i = 0; i < chi2Points.length; i++) {
    const sx = toScreenX(chi2Points[i].x);
    const sy = toScreenY(chi2Points[i].y);
    if (i === 0) ctx.moveTo(sx, sy);
    else ctx.lineTo(sx, sy);
  }
  ctx.stroke();

  // --- 5. Draw Lower & Upper Cutoff Lines ---
  const drawCutoffLine = (cutoffVal, labelText, color) => {
    const sx = toScreenX(cutoffVal);
    if (sx >= padding.left && sx <= padding.left + width) {
      ctx.beginPath();
      ctx.strokeStyle = color;
      ctx.setLineDash([3, 3]);
      ctx.lineWidth = 1.5;
      ctx.moveTo(sx, padding.top);
      ctx.lineTo(sx, padding.top + height);
      ctx.stroke();
      ctx.setLineDash([]);

      ctx.fillStyle = color;
      ctx.font = '600 10px system-ui, sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(`${labelText} (${cutoffVal.toFixed(1)})`, sx, padding.top + height + 32);
    }
  };

  drawCutoffLine(lowerCutoff, 'Lower Cutoff', '#f59e0b');
  drawCutoffLine(upperCutoff, 'Upper Cutoff', '#f43f5e');

  // --- 6. Draw Sample Chi-Square Marker Dot ---
  if (options.chiSquare != null && options.chiSquare >= xMin && options.chiSquare <= xMax) {
    const sampleSx = toScreenX(options.chiSquare);
    
    // Find closest y value on curve
    let sampleY = 0;
    let minDiff = Infinity;
    for (const p of chi2Points) {
      const diff = Math.abs(p.x - options.chiSquare);
      if (diff < minDiff) {
        minDiff = diff;
        sampleY = p.y;
      }
    }
    const sampleSy = toScreenY(sampleY);

    // Draw vertical marker line
    ctx.beginPath();
    ctx.strokeStyle = '#e2e8f0';
    ctx.lineWidth = 2;
    ctx.moveTo(sampleSx, padding.top);
    ctx.lineTo(sampleSx, padding.top + height);
    ctx.stroke();

    // Glowing Dot
    ctx.beginPath();
    ctx.arc(sampleSx, sampleSy, 6, 0, Math.PI * 2);
    ctx.fillStyle = '#38bdf8';
    ctx.shadowColor = '#38bdf8';
    ctx.shadowBlur = 10;
    ctx.fill();
    ctx.shadowBlur = 0;
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 2;
    ctx.stroke();

    // Sample Value Badge
    ctx.fillStyle = '#ffffff';
    ctx.font = '700 11px system-ui, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(`Sample χ² = ${options.chiSquare.toFixed(2)}`, sampleSx, padding.top - 8);
  }

  ctx.restore();
}
