const fps = new (class {
  constructor() {
    // Grab the DOM element for the FPS counter.
    this.fps = document.getElementById("fps");
    // Init a list of frames per second for the last 100 frames.
    this.frames = [];
    // At construction time, the last frame timestamp is now.
    this.lastFrameTimeStamp = performance.now();
  }

  

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    // The delta time is the difference between now and the last frame, where
    // at the very beginning, right after construction of the element, the
    // last frame is the now from then.
    const delta = now - this.lastFrameTimeStamp;
    // Overwrite the last frame timestamp with the current one.
    this.lastFrameTimeStamp = now;
    // Calculate the FPS from the delta. This is done by taking the reciprocal
    // of the delta, which is the number of frames per millisecond, and then
    // multiplying by 1000 to get the number of frames per second.
    const fps = (1 / delta) * 1000;

    // Save only the latest 100 timings. We push the new timing onto the end
    // of the array, and then shift off the first element if the array is
    // longer than 100 elements.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    // For comparison, we use infinity and -infinity as the initial values
    // for min and max, respectively. This way, any value will be smaller
    // or larger than these values, and will thus be set as the new min
    // or max.
    let min = Infinity;
    let max = -Infinity;
    // We also keep a running sum of all the frames, which we will divide
    // by the number of frames to get the mean.
    let sum = 0;
    // For each frame in the stack, we check if it is the new min or max,
    // might replace the old value with the new minimum or maximum, and add
    // it to the sum.
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    // Calculate the mean FPS.
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
  Frames per second:
LATEST = ${Math.round(fps)}
AVG of last 100 = ${Math.round(mean)}
MIN of last 100 = ${Math.round(min)}
MAX of last 100 = ${Math.round(max)}
  `.trim();
  }
})();

export { fps };
