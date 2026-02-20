# Session: 2026-02-01 16:40:50 UTC

- **Session Key**: agent:main:main
- **Session ID**: 78f4a32e-a5e4-413c-8a85-a31025ab0d93
- **Source**: whatsapp

## Conversation Summary

assistant: I opened the **Cashflow** tab (Jan–Jul 2026) and pulled the numbers. Here’s a clean breakdown + a few “what‑if” monthly cashflow scenarios.

ASSUMPTIONS (so you can sanity-check)
- “Cashflow” tab looks like **invoices paid (cash in)**, not work done.
- I treated these as cash items **only where amounts are filled** on that tab.
- I also used the explicit **Jan one-offs/debt lines** shown near the bottom.
- Partner distributions are shown as **“TOTAL A+B+C” (to each partner)**; I treat that as an *optional draw lever* (can be deferred if cash is tight).

## 1) Base plan (as sheet): expected cash-in by month
Total revenue (A+B+C+D):
- **Jan:** £37,975
- **Feb:** £53,716
- **Mar:** £138,716
- **Apr:** £148,716
- **May:** £182,716
- **Jun:** £76,000
- **Jul:** £77,000

Where it’s coming from (high level)
- **Already invoiced / due:** Cadro pilot £13,700 (Jan), Charm MS4 £9,275 (Jan), Charm Stage 3/3A £15,000 (Jan)
- **Won:** **ODILO** £37,716/month (Feb–May)
- **Proposals / uncertain:** Charm Stage 4A/4B, Cadro migration/maintenance, Biorelate, Klarian, Cadro v2, Unravel (funding-dependent)

## 2) Big cash-out items explicitly listed (Jan)
From the bottom section of Cashflow:
- **Jan one-offs total:** **£37,000** (BUPA £7.5k + Juras £2k + corp tax reserve £10k + expenses £7.5k + IP loan £17.5k)
- **Jan debt to Zestic AI (Alex M):** **£27,000**
- So **Jan fixed-ish out** shown = **£64,000**

This is why January is the pinch month in most scenarios.

## 3) Monthly net cash movement — Base case
I’m showing:
- **Net (pre-partner draws)** = Revenue – (Jan one-offs + Jan debt only)
- **Net (after partner draws)** = Net(pre) – partner distribution (the sheet’s “TOTAL A+B+C”)

Base case net:
- **Jan:** net(pre) **-£26,025** ; net(after partner draws) **-£33,145**
- **Feb:** net(pre) **£53,716** ; net(after) **£47,448**
- **Mar:** net(pre) **£138,716** ; net(after) **£121,132**
- **Apr:** net(pre) **£148,716** ; net(after) **£130,570**
- **May:** net(pre) **£182,716** ; net(after) **£161,261**
- **Jun:** net(pre) **£76,000** ; net(after) **£67,394**
- **Jul:** net(pre) **£77,000** ; net(after) **£68,281**

Key takeaway: **if you actually pay the £64k Jan outgoings in Jan, you need either starting cash or to defer something** (partner draws, debt repayment timing, or some one-offs).

## 4) What-if scenarios (monthly cashflow)

### Scenario A — “Committed only”
Include only:
- the **Jan invoices due** (Cadro pilot + Charm MS4 + Charm Stage 3/3A)
- **ODILO (won)** (Feb–May)
Exclude all “preparing proposal / trying to recover / funding unlock” items.

Cash-in:
- Jan £37,975 | Feb £37,716 | Mar £37,716 | Apr £37,716 | May £37,716 | Jun £0 | Jul £0

Net (pre-partner draws):
- **Jan -£26,025**
- **Feb +£37,716**
- **Mar +£37,716**
- **Apr +£37,716**
- **May +£37,716**
- **Jun £0**
- **Jul £0**

Meaning: you’d be fine Feb–May, but **you hit a cliff in Jun/Jul unless pipeline lands or costs are cut**.

### Scenario B — “Probability-weighted pipeline”
A reasonable risk-adjusted view (you can change these %s):
- 100%: Jan invoices + ODILO
- 50%: Charm 4A/4B + Cadro migration/maintenance + Biorelate
- 30%: Klarian + Cadro v2
- 20%: Unravel (funding-dependent)

Cash-in comes out roughly:
- **Jan £37,975 | Feb £45,716 | Mar £80,216 | Apr £79,216 | May £89,216 | Jun £26,000 | Jul £26,500**

Net (pre-partner draws):
- **Jan -£26,025**
- **Feb +£45,716**
- **Mar +£80,216**
- **Apr +£79,216**
- **May +£89,216**
- **Jun +£26,000**
- **Jul +£26,500**

Meaning: still January pinch, but the rest of the period stays cash-positive even with conservative probabilities.

### Scenario C — “1-month payment delay on everything”
This models customers paying 30 days later than expected.

Cash-in shifts right (Jan becomes £0, Feb gets Jan’s cash, etc.).
Impact:
- **January becomes the real danger month**: with the £64k Jan cash-outs, you’re **~£64k down before any cash arrives** (and ~£71k down if partner draws still happen).

This is the scenario I’d plan liquidity around.

## 5) Quick recommendations (based on the sheet)
- Treat **partner draws as a dial**, not a default: if Jan is tight, defer Jan draw(s) until Feb cash lands.
- Decide which of these must happen in Jan: **£27k Alex M repayment**, **£10k tax reserve**, **£17.5k IP loan**. Moving even one of them changes the whole picture.
- If you want a “true company cashflow”, we should also pull recurring costs from the **Costs** tab (Google/Xero/accounting/etc.) and any payroll—right now the Cashflow tab only includes a few specific outflows.

If you tell me:
1) Starting bank balance at end of Jan 2025 / start of Jan 2026 (even rough), and  
2) Whether **partner distributions are optional** month-to-month,  
…I can turn this into a proper month-by-month **closing cash balance** under each scenario (best/base/worst + payment delays).
