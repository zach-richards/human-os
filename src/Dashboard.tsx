import { useState } from "react";
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid } from "recharts";

// ✏️ Change this to any color you want
const GRAPH_COLOR = "#416822";

// Build the last hour of data with timestamps
const now = Date.now();
const data = [
  { time: new Date(now - 60 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 20 },
  { time: new Date(now - 45 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 55 },
  { time: new Date(now - 30 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 30 },
  { time: new Date(now - 15 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 75 },
  { time: new Date(now).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),                  score: 50 },
];

const activities = ["Coding", "Writing", "School", "Work", "Free"];

export default function FocusChart() {
  const [activity, setActivity] = useState("");

  return (
    <div style={{ padding: 24, maxWidth: 600, fontFamily: "sans-serif" }}>

      {/* Header */}
      <div style={{ marginBottom: 20 }}>
        <h2 style={{ margin: 0, fontSize: 20 }}>Focus Score</h2>
        <p style={{ margin: "4px 0 0", color: "#888", fontSize: 14 }}>Last 60 minutes</p>
      </div>

      {/* Graph */}
      <div style={{ background: "#242526", borderRadius: 12, padding: "16px 8px 8px" }}>
        <ResponsiveContainer width="100%" height={250}>
          <AreaChart data={data} margin={{ top: 0, right: 16, left: -16, bottom: 0 }}>
            <CartesianGrid stroke="#ececec" vertical={false} />
            <XAxis dataKey="time" tick={{ fontSize: 12, fill: "#888" }} axisLine={false} tickLine={false} />
            <YAxis domain={[0, 100]} tick={{ fontSize: 12, fill: "#888" }} axisLine={false} tickLine={false} />
            <Tooltip
              contentStyle={{ borderRadius: 8, border: "none", boxShadow: "0 2px 8px rgba(0,0,0,0.1)" }}
              formatter={(value) => [`${value ?? ""}`, "Score"]}
            />
            <defs>
              <linearGradient id="colorGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor={GRAPH_COLOR} stopOpacity={0.3} />
                <stop offset="100%" stopColor={GRAPH_COLOR} stopOpacity={0} />
              </linearGradient>
            </defs>
            <Area
              type="monotone"
              dataKey="score"
              stroke={GRAPH_COLOR}
              strokeWidth={2}
              fill="url(#colorGrad)"
              dot={{ fill: GRAPH_COLOR, r: 4, strokeWidth: 0 }}
              activeDot={{ r: 6 }}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      {/* Dropdown */}
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
            border: "1px solid #242526",
            fontSize: 14,
            background: "#242526",
            cursor: "pointer",
          }}
        >
          <option value="">Select an activity...</option>
          {activities.map((a) => (
            <option key={a} value={a}>{a}</option>
          ))}
        </select>
      </div>

      {/* Selected activity */}
      {activity && (
        <p style={{ marginTop: 12, fontSize: 14, color: "#ddd" }}>
          Currently tracking: <strong>{activity}</strong>
        </p>
      )}

    </div>
  );
}