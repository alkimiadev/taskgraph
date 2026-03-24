# Cost-Benefit Framework

TaskGraph uses categorical estimates (risk: low/medium/high) rather than numeric costs. This document explains why.

## The Framework

The expected cost of a task:

```
EV_task = P_success × C_success + (1 - P_success) × C_fail
```

Where:
- `P_success` = probability the task completes successfully
- `C_success` = cost when it works (API fees, time, tokens)
- `C_fail` = cost when it fails (human intervention, rework, context switching)

**Key insight:** Leaderboards optimize for accuracy under infinite patience. Your wallet optimizes for accuracy under finite money and time.

## Why Upstream Quality Matters

Failures at higher levels multiply downstream surface area:

```
planning failure → wrong decomposition → wasted implementation
decomposition failure → unclear tasks → rework
review failure → bugs shipped → rework
```

This means `risk: critical` at planning level > `risk: critical` at implementation level.

### Example: Planning Quality Impact

| Scenario | Planning p | Total Cost |
|----------|------------|------------|
| Good planning | 0.92 | $12.29 |
| Poor planning | 0.65 | $30.71 |

**Result:** Poor planning increases total cost by 150%, even with identical implementation tasks.

The failure propagates: poor planning reduces decomposition quality, which reduces implementation effectiveness, which increases integration issues.

## Why Categorical Estimates

### 1. LLM Calibration

Models reliably distinguish "high risk vs medium risk" but struggle with "$3.42 vs $3.50".

**Example:** Ask an LLM to estimate task cost:
- Numeric: "This will take 2.5 hours and cost $4.20" → often wrong
- Categorical: "This is moderate scope with medium risk" → usually correct

### 2. Environment Independence

Token counts, model costs, and verbosity vary by:
- Model (GPT-4 vs Claude vs local)
- Provider (OpenAI vs Azure vs Anthropic)
- Context window usage
- Output format requirements

Categorical estimates remain valid across environments. "Medium risk" means the same thing whether you're using GPT-4 or Claude.

### 3. Relative Ordering Over Precision

We need to know: Task A > Task B (A is riskier)

We don't need: Task A = $3.42, Task B = $2.87

The `decompose` command flags tasks where `risk > medium` OR `scope > moderate`. This structural insight matters more than precise dollar amounts.

## The Kuhn Poker Analogy

[Kuhn Poker](https://en.wikipedia.org/wiki/Kuhn_poker) is a simplified 3-card poker game. Despite being trivially simple, it reveals a fundamental truth about zero-sum imperfect information games: **you must bluff**.

This structural property holds in complex games like No-Limit Texas Hold'em. The toy model isolates the essential truth.

Similarly, this framework is a toy model—it ignores many real-world factors. But it reveals structural truths:

1. **You must plan** - Skipping planning increases expected cost
2. **You must decompose** - Large tasks have higher failure rates
3. **You must review** - Upstream errors compound downstream

These aren't optional optimizations. They're structural requirements, like bluffing in poker.

## Mappings (For Relative Comparison)

TaskGraph uses these defaults for internal calculations:

**Risk → Failure Likelihood:**
| Risk | Ordering | Implied p |
|------|----------|-----------|
| trivial | 1 | 0.98 |
| low | 2 | 0.90 |
| medium | 3 | 0.80 |
| high | 4 | 0.65 |
| critical | 5 | 0.50 |

**Scope → Size:**
| Scope | Ordering | Approximate Tokens |
|-------|----------|-------------------|
| single | 1 | ~500 |
| narrow | 2 | ~1,500 |
| moderate | 3 | ~3,000 |
| broad | 4 | ~6,000 |
| system | 5 | ~10,000+ |

**Impact → Downstream Damage:**
| Impact | Ordering | Weight |
|--------|----------|--------|
| isolated | 1 | 1.0 |
| component | 2 | 1.5 |
| phase | 3 | 2.0 |
| project | 4 | 3.0 |

**Note:** These are reasonable defaults for relative comparison. They should not be interpreted as precise measurements.

## References

- **Colab Notebook:** [Cost-Benefit Analysis Framework](https://colab.research.google.com/drive/1_Vqa-qH-0WfZh8ngtet-l9bN-g1ILRKT)
- **Local Copy:** [cost_benefit_analysis_framework.py](./research/cost_benefit_analysis_framework.py)