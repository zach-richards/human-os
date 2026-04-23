import React, { useState } from "react";
import {
  Box,
  Paper,
  Typography,
  List,
  ListItemButton,
  ListItemText,
  Divider,
  Select,
  MenuItem,
  TextField,
  Button
} from "@mui/material";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  ResponsiveContainer
} from "recharts";

// ---------- Types ----------
type Priority = {
  id: number;
  name: string;
  value: number;
};

// ---------- Data ----------
const initialPriorities: Priority[] = [
  { id: 1, name: "Health", value: 8 },
  { id: 2, name: "Work / Study", value: 9 },
  { id: 3, name: "Learning AI", value: 10 },
  { id: 4, name: "Social", value: 6 },
  { id: 5, name: "Fitness", value: 7 },
];

function generateFocusFuel(category: string) {
  const base: Record<string, number[]> = {
    Programming: [6, 7, 8, 7, 9, 8, 10],
    School: [8, 8, 7, 6, 7, 8, 7],
  };

  const values =
    base[category] ||
    Array.from({ length: 7 }, (_, i) => {
      const seed = category
        .split("")
        .reduce((a, c) => a + c.charCodeAt(0), 0);

      return ((seed + i * 3) % 10) + 1;
    });

  return values.map((v, i) => ({
    day: `Day ${i + 1}`,
    fuel: v,
  }));
}

// ---------- Component ----------
export default function HumanOSDashboard() {
  const [selected, setSelected] = useState<Priority | null>(null);
  const [priorities] = useState<Priority[]>(initialPriorities);

  const [categories, setCategories] = useState<string[]>([
    "Programming",
    "School",
  ]);
  const [selectedCategory, setSelectedCategory] = useState("Programming");
  const [newCategory, setNewCategory] = useState("");

  const data = generateFocusFuel(selectedCategory);

  const addCategory = () => {
    const cleaned = newCategory.trim();
    if (!cleaned) return;

    if (!categories.includes(cleaned)) {
      setCategories((prev) => [...prev, cleaned]);
      setSelectedCategory(cleaned);
    }

    setNewCategory("");
  };

  return (
    <Box sx={{ display: "flex", height: "100vh", bgcolor: "#0f172a", color: "white" }}>
      {/* Sidebar */}
      <Paper sx={{ width: 300, p: 2, bgcolor: "#111827", color: "white" }}>
        <Typography variant="h6" sx={{ mb: 2 }}>
          Priorities
        </Typography>
        <Divider sx={{ bgcolor: "gray" }} />
        <List>
          {priorities.map((item) => (
            <ListItemButton
              key={item.id}
              selected={selected?.id === item.id}
              onClick={() => setSelected(item)}
            >
              <ListItemText
                primary={item.name}
                secondary={`Score: ${item.value}`}
              />
            </ListItemButton>
          ))}
        </List>
      </Paper>

      {/* Main Content */}
      <Box sx={{ flex: 1, p: 3 }}>
        <Typography variant="h5" sx={{ mb: 2 }}>
          Focus Fuel Graph
        </Typography>

        <Paper sx={{ p: 2, height: 400, bgcolor: "#111827" }}>
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="day" />
              <YAxis />
              <Tooltip />
              <Line type="monotone" dataKey="fuel" stroke="#38bdf8" />
            </LineChart>
          </ResponsiveContainer>
        </Paper>

        {/* Controls */}
        <Paper
          sx={{
            mt: 2,
            p: 2,
            bgcolor: "#111827",
            display: "flex",
            gap: 2,
            alignItems: "center",
          }}
        >
          <Typography>Focus Mode:</Typography>

          <Select
            value={selectedCategory}
            onChange={(e) => setSelectedCategory(e.target.value)}
            sx={{
              color: "white",
              minWidth: 150,
              "& .MuiSvgIcon-root": { color: "white" },
            }}
          >
            {categories.map((cat) => (
              <MenuItem key={cat} value={cat} sx={{ color: "black" }}>
                {cat}
              </MenuItem>
            ))}
          </Select>

          <TextField
            size="small"
            value={newCategory}
            onChange={(e) => setNewCategory(e.target.value)}
            placeholder="Add category"
            sx={{
              minWidth: 160,
              input: { color: "white" },
            }}
          />

          <Button variant="contained" onClick={addCategory}>
            Add
          </Button>
        </Paper>

        {/* Selected priority */}
        {selected && (
          <Paper sx={{ mt: 3, p: 2, bgcolor: "#111827" }}>
            <Typography variant="h6">Selected Priority</Typography>
            <Typography>Name: {selected.name}</Typography>
            <Typography>Score: {selected.value}</Typography>
          </Paper>
        )}
      </Box>
    </Box>
  );
}