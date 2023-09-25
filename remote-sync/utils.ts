export function prettyPrintDuration(duration) {
  if (duration >= 1000) {
    return (duration / 1000).toFixed(0) + "ms";
  } else {
    return duration.toFixed(0) + "us";
  }
}
