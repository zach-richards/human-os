// Dashboard.tsx

// Displays a graph of focus over the last 5 minutes and an activity selector.

import { useEffect, useState } from "react";
import {
  AreaChart, Area, XAxis, YAxis, Tooltip,
  ResponsiveContainer, CartesianGrid,
} from "recharts";
import { scores, subscribeToScores } from "./communication/TSXReciever";
import { getSampled } from "./sampling";

const GRAPH_COLOR = "#6366f1";
const POINTS = 6; // current + 5 historical minutes
const activities = ["Coding", "Writing", "School", "Work", "Free"];

function buildChartData(pointCount: number) {
  const sampled = getSampled(scores, pointCount);

  // Show a flat zero baseline until enough data arrives
  const source: number[] = sampled ?? Array(pointCount).fill(0);

  const now = Date.now();

  return source.map((score, i) => {
    const minutesAgo = source.length - 1 - i;
    return {
      time: new Date(now - minutesAgo * 60 * 1000).toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
      }),
      score,
    };
  });
}

export default function Dashboard() {
  // Re-render whenever a new score arrives from the backend
  const [, forceUpdate] = useState(0);
  useEffect(() => {
    return subscribeToScores(() => forceUpdate((n) => n + 1));
  }, []);

  const [activity, setActivity] = useState("");

  const data = buildChartData(POINTS);
  const currentScore = scores[scores.length - 1] ?? 0;

  return (
    <div style={{ padding: 24, maxWidth: 600, fontFamily: "sans-serif" }}>

      {/* Header */}
      <div style={{ marginBottom: 20, display: "flex", alignItems: "baseline", gap: 12 }}>
        <h2 style={{ margin: 0, fontSize: 20 }}>Focus Score</h2>
        <span style={{ fontSize: 28, fontWeight: 700, color: GRAPH_COLOR }}>
          {currentScore}
        </span>
        <p style={{ margin: 0, color: "#888", fontSize: 14, marginLeft: "auto" }}>
          Last 5 minutes
        </p>
      </div>

      {/* Graph */}
      <div style={{ background: "#242526", borderRadius: 12, padding: "16px 8px 8px" }}>
        <ResponsiveContainer width="100%" height={250}>
          <AreaChart data={data} margin={{ top: 0, right: 16, left: -16, bottom: 0 }}>
            <defs>
              <linearGradient id="colorGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor={GRAPH_COLOR} stopOpacity={0.3} />
                <stop offset="100%" stopColor={GRAPH_COLOR} stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid stroke="#333" vertical={false} />
            <XAxis
              dataKey="time"
              tick={{ fontSize: 12, fill: "#888" }}
              axisLine={false}
              tickLine={false}
            />
            <YAxis
              domain={[0, 100]}
              tick={{ fontSize: 12, fill: "#888" }}
              axisLine={false}
              tickLine={false}
            />
            <Tooltip
              contentStyle={{
                borderRadius: 8,
                border: "none",
                boxShadow: "0 2px 8px rgba(0,0,0,0.3)",
                backgroundColor: "#3A3B3C",
                color: "#fff",
              }}
              formatter={(value) => [`${value ?? ""}`, "Score"]}
            />
            <Area
              type="monotone"
              dataKey="score"
              stroke={GRAPH_COLOR}
              strokeWidth={2}
              fill="url(#colorGrad)"
              dot={{ fill: GRAPH_COLOR, r: 4, strokeWidth: 0 }}
              activeDot={{ r: 6 }}
              isAnimationActive={false}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      {/* Activity selector */}
      <div style={{ marginTop: 20 }}>
        <label style={{ fontSize: 14, color: "#ddd", display: "block", marginBottom: 6 }}>
          Activity
        </label>
        <select
          value={activity}
          onChange={(e) => setActivity(e.target.value)}
          style={{
            width: "100%",
            padding: "10px 12px",
            borderRadius: 8,
            border: "1px solid #444",
            fontSize: 14,
            background: "#242526",
            color: "#ddd",
            appearance: "none",
            cursor: "pointer",
          }}
        >
          <option value="">Select an activity...</option>
          {activities.map((a) => (
            <option key={a} value={a}>{a}</option>
          ))}
        </select>
      </div>

      {activity && (
        <p style={{ marginTop: 12, fontSize: 14, color: "#ddd" }}>
          Currently tracking: <strong>{activity}</strong>
        </p>
      )}

    </div>
  );
}