/**
 * Court section definitions for rendering players on the basketball court.
 * Coordinates are in percentages of the canvas dimensions.
 */

export interface CourtSection {
  name: string;
  x: number;
  y: number;
  w: number;
  h: number;
  color: string;
}

/**
 * Default section color for debugging visualization.
 */
const COLORS = {
  red: "rgba(255, 0, 0, 0.5)",
  yellow: "rgba(255, 255, 0, 0.5)",
  green: "rgba(0, 255, 0, 0.5)",
  blue: "rgba(0, 0, 255, 0.5)",
  cyan: "rgba(0, 255, 255, 0.5)",
};

/**
 * Court sections for the home team's offensive side (basket on the left).
 * Each section corresponds to a CourtArea enum value from the Rust backend.
 */
export const COURT_SECTIONS: CourtSection[] = [
  // Right side of court (top)
  { name: "ThreePointLineCornerRight", x: 0, y: 0, w: 15, h: 12, color: COLORS.red },
  { name: "ThreePointLineWingRight", x: 15, y: 0, w: 16, h: 24, color: COLORS.yellow },
  { name: "MidrangeBaselineRight", x: 0, y: 12, w: 7, h: 16, color: COLORS.green },
  { name: "MidrangeWingRight", x: 7, y: 12, w: 8, h: 16, color: COLORS.blue },
  { name: "ShortCornerRight", x: 0, y: 28, w: 6, h: 11, color: COLORS.blue },
  { name: "LowPostRight", x: 6, y: 28, w: 9, h: 11, color: COLORS.yellow },
  { name: "RestrictedAreaRight", x: 0, y: 39, w: 8, h: 6, color: COLORS.red },
  { name: "ElbowRight", x: 15, y: 28, w: 5, h: 11, color: COLORS.red },

  // Center areas
  { name: "FreeThrowLine", x: 12, y: 39, w: 8, h: 22, color: COLORS.green },
  { name: "RestrictedAreaMiddle", x: 7, y: 45, w: 4, h: 11, color: COLORS.green },
  { name: "MidrangeCenter", x: 20, y: 24, w: 7, h: 52, color: COLORS.blue },
  { name: "ThreePointLineCenter", x: 27, y: 24, w: 11, h: 52, color: COLORS.green },
  { name: "Center", x: 38, y: 0, w: 25, h: 100, color: COLORS.red },
  { name: "Backcourt", x: 63, y: 0, w: 37, h: 100, color: COLORS.yellow },

  // Left side of court (bottom)
  { name: "ThreePointLineCornerLeft", x: 0, y: 88, w: 15, h: 12, color: COLORS.yellow },
  { name: "ThreePointLineWingLeft", x: 15, y: 76, w: 16, h: 24, color: COLORS.red },
  { name: "MidrangeBaselineLeft", x: 0, y: 72, w: 7, h: 16, color: COLORS.green },
  { name: "MidrangeWingLeft", x: 7, y: 74, w: 8, h: 14, color: COLORS.blue },
  { name: "ShortCornerLeft", x: 0, y: 61, w: 6, h: 11, color: COLORS.blue },
  { name: "LowPostLeft", x: 6, y: 61, w: 9, h: 11, color: COLORS.yellow },
  { name: "RestrictedAreaLeft", x: 0, y: 56, w: 8, h: 5, color: COLORS.cyan },
  { name: "ElbowLeft", x: 15, y: 61, w: 5, h: 11, color: COLORS.red },
];

/**
 * Flips section coordinates horizontally for when possession changes.
 * This allows rendering players on the opposite side of the court.
 */
export function flipSections(sections: CourtSection[]): CourtSection[] {
  return sections.map(section => ({
    ...section,
    x: 100 - section.x - section.w,
  }));
}

/**
 * Converts a percentage value to pixels based on the total dimension.
 */
export function percentToPixels(percent: number, total: number): number {
  return (percent / 100) * total;
}

/**
 * Finds a section by its name.
 */
export function findSection(
  sections: CourtSection[],
  name: string
): CourtSection | undefined {
  return sections.find(section => section.name === name);
}
