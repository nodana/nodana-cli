const MINS_IN_DAY = 1440;
const MINS_IN_HOUR = 60;

export const sleep = (delay: number) => {
  return new Promise((resolve) => setTimeout(resolve, delay));
};

// Shows mins in hours and minutes
export const getDurationString = (mins: number) => {
  if (!mins || !Number.isInteger(mins)) {
    return "Not available";
  }

  const parts: string[] = [];

  let d = Math.floor(mins / MINS_IN_DAY);
  let h = Math.floor((mins - d * MINS_IN_DAY) / MINS_IN_HOUR);
  let m = mins % MINS_IN_HOUR;

  if (d > 0) {
    parts.push(`${d} days`);
  }

  if (h > 0) {
    parts.push(`${h} hours`);
  }

  if (m > 0) {
    parts.push(`${m} minutes`);
  }

  return parts.join(", ");
};
