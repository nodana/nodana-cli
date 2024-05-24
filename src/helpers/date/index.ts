export const sleep = (delay: number) => {
  return new Promise((resolve) => setTimeout(resolve, delay));
};

// Shows mins in hours and minutes
export const getDurationString = (mins: number) => {
  if (!mins || !Number.isInteger(mins)) {
    return "Duration not available";
  }

  if (mins < 60) {
    return `${mins} minutes`;
  }

  const h = Math.floor(mins / 60);
  const m = mins % 60;

  return `${h} hours, ${m} minutes`;
};
