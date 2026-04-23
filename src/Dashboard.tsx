import { useState } from "react";
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid
} from "recharts";

import { COLORS, RGB, toRgbString } from "./color";

type FocusChartProps = {
  color: RGB;
};

const now = Date.now();

const data = [
  { time: new Date(now - 60 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 20 },
  { time: new Date(now - 45 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 55 },
  { time: new Date(now - 30 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 30 },
  { time: new Date(now - 15 * 60 * 1000).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 75 },
  { time: new Date(now).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }), score: 50 },
];

const activities = ["Coding", "Writing", "School", "Work", "Free"];

export default function FocusChart({ color }: FocusChartProps) {
  const [activity, setActivity] = useState("");

  const graphColor = toRgbString(color);

  return (
    <div style={{ padding: 24, maxWidth: 600, fontFamily: "sans-serif" }}>

      <div style={{ marginBottom: 20 }}>
        <h2 style={{ margin: 0, fontSize: 20, color: "#E4E6EB" }}>
          Focus Fuel
        </h2>
        <p style={{ margin: "4px 0 0", color: "#888", fontSize: 14 }}>
          Last 60 minutes
        </p>
      </div>

      <div style={{ background: "#242526", borderRadius: 12, padding: "16px 8px 8px" }}>
        <ResponsiveContainer width="100%" height={250}>
          <AreaChart data={data}>
            <CartesianGrid vertical={false} />
            <XAxis dataKey="time" />
            <YAxis domain={[0, 100]} />
            <Tooltip />

            <defs>
              <linearGradient id="colorGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor={graphColor} stopOpacity={0.3} />
                <stop offset="100%" stopColor={graphColor} stopOpacity={0} />
              </linearGradient>
            </defs>

            <Area
              type="monotone"
              dataKey="score"
              stroke={graphColor}
              fill="url(#colorGrad)"
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      <select
        value={activity}
        onChange={(e) => setActivity(e.target.value)}
      >
        <option value="">Select an activity...</option>
        {activities.map((a) => (
          <option key={a} value={a}>{a}</option>
        ))}
      </select>

    </div>
  );
}