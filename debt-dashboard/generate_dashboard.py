import csv
import json
from pathlib import Path

BASE = Path(__file__).resolve().parent


def load_rows(path: Path):
    with path.open() as f:
        return list(csv.DictReader(f))


def num(x):
    try:
        return float(x)
    except Exception:
        return 0.0


rows = load_rows(BASE / "debts.csv")

for r in rows:
    r["owed_gbp"] = num(r.get("owed_gbp"))
    r["offer_monthly_gbp"] = num(r.get("offer_monthly_gbp"))
    r["priority"] = (r.get("priority") or "no").strip().lower()


total_owed = sum(r["owed_gbp"] for r in rows)
priority_owed = sum(r["owed_gbp"] for r in rows if r["priority"] == "yes")
nonpriority_owed = total_owed - priority_owed
total_offer = sum(r["offer_monthly_gbp"] for r in rows)

labels = [r["name"] for r in rows]
owed = [r["owed_gbp"] for r in rows]
offers = [r["offer_monthly_gbp"] for r in rows]


def money(x: float) -> str:
    return f"£{x:,.2f}"


# Build table rows safely
trs = []
for r in rows:
    pill_class = "yes" if r["priority"] == "yes" else "no"
    pill_label = "priority" if r["priority"] == "yes" else "non-priority"
    trs.append(
        "<tr>"
        f"<td>{r['name']}</td>"
        f"<td><span class='pill {pill_class}'>{pill_label}</span></td>"
        f"<td>{money(r['owed_gbp'])}</td>"
        f"<td>{money(r['offer_monthly_gbp'])}</td>"
        f"<td class='muted'>{r.get('notes','')}</td>"
        "</tr>"
    )

html = """<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Debt Dashboard</title>
  <script src="https://cdn.plot.ly/plotly-2.30.0.min.js"></script>
  <style>
    body { font-family: system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif; margin: 24px; background:#0b0f14; color:#e8eef6; }
    h1,h2 { margin: 0 0 10px 0; }
    .muted { color: rgba(232,238,246,.75); }
    .grid { display:grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 12px; margin: 14px 0 18px; }
    .card { background: rgba(255,255,255,.04); border: 1px solid rgba(255,255,255,.12); border-radius: 14px; padding: 12px 14px; }
    .big { font-size: 22px; font-weight: 700; }
    table { width: 100%; border-collapse: collapse; }
    th, td { border-bottom: 1px solid rgba(255,255,255,.12); padding: 8px 10px; font-size: 14px; text-align:left; vertical-align: top; }
    th { color: rgba(232,238,246,.8); font-weight: 600; }
    .pill { display:inline-block; padding: 2px 8px; border-radius:999px; font-size: 12px; border:1px solid rgba(255,255,255,.18); }
    .yes { background: rgba(120,255,170,.12); border-color: rgba(120,255,170,.35); }
    .no { background: rgba(255,190,120,.10); border-color: rgba(255,190,120,.35); }
    code { background: rgba(255,255,255,.08); padding: 2px 6px; border-radius: 8px; }
  </style>
</head>
<body>
  <h1>Debt Dashboard</h1>
  <div class="muted">Source: Financial statement PDF (27 Jan 2026). Edit <code>debts.csv</code> to update.</div>

  <div class="grid">
    <div class="card"><div class="muted">Total owed</div><div class="big">__TOTAL_OWED__</div></div>
    <div class="card"><div class="muted">Priority owed</div><div class="big">__PRIORITY_OWED__</div></div>
    <div class="card"><div class="muted">Non-priority owed</div><div class="big">__NONPRIORITY_OWED__</div></div>
    <div class="card"><div class="muted">Current monthly offers</div><div class="big">__TOTAL_OFFER__</div></div>
  </div>

  <div class="card" style="margin-bottom:12px;">
    <h2>By debt (owed)</h2>
    <div id="pie" style="height:360px;"></div>
  </div>

  <div class="card" style="margin-bottom:12px;">
    <h2>Monthly offers</h2>
    <div id="bar" style="height:420px;"></div>
  </div>

  <div class="card">
    <h2>Debt list</h2>
    <table>
      <thead>
        <tr>
          <th>Debt</th>
          <th>Priority</th>
          <th>Owed</th>
          <th>Offer / month</th>
          <th>Notes</th>
        </tr>
      </thead>
      <tbody>
        __TABLE_ROWS__
      </tbody>
    </table>
  </div>

<script>
  const labels = __LABELS__;
  const owed = __OWED__;
  const offers = __OFFERS__;

  Plotly.newPlot('pie', [{
    type: 'pie',
    labels, values: owed,
    textinfo: 'label+percent',
    insidetextorientation: 'radial'
  }], {
    paper_bgcolor: 'rgba(0,0,0,0)',
    plot_bgcolor: 'rgba(0,0,0,0)',
    font: {color: '#e8eef6'},
    margin: {t: 10, r: 10, b: 10, l: 10}
  }, {displayModeBar: false, responsive: true});

  Plotly.newPlot('bar', [{
    type: 'bar',
    x: labels,
    y: offers,
    name: 'Monthly offer',
    marker: {color: 'rgba(135,183,255,.9)'}
  }], {
    paper_bgcolor: 'rgba(0,0,0,0)',
    plot_bgcolor: 'rgba(0,0,0,0)',
    font: {color: '#e8eef6'},
    yaxis: {title: '£ per month'},
    xaxis: {tickangle: -20},
    margin: {t: 10, r: 10, b: 90, l: 60}
  }, {displayModeBar: false, responsive: true});
</script>
</body>
</html>
"""

html = (
    html.replace("__TOTAL_OWED__", money(total_owed))
    .replace("__PRIORITY_OWED__", money(priority_owed))
    .replace("__NONPRIORITY_OWED__", money(nonpriority_owed))
    .replace("__TOTAL_OFFER__", money(total_offer))
    .replace("__TABLE_ROWS__", "\n".join(trs))
    .replace("__LABELS__", json.dumps(labels))
    .replace("__OWED__", json.dumps(owed))
    .replace("__OFFERS__", json.dumps(offers))
)

out = BASE / "dashboard.html"
out.write_text(html)
print(str(out))
