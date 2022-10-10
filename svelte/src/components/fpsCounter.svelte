<script lang="ts">
  import { showFps } from "../lib/stores";

  var someNumber = 0;
  var min = 0;
  var max = 0;
  var mean = 0;
  var fps = NaN;

  // We need the current timestamp for the calculation of the FPS
  // via the delta later on.
  let lastFrameTimeStamp = performance.now();

  // We have to keep track of the last n frames in order
  // to calculate a mean, max and min.
  let frames: any[] = [];

  export function renderFpsComponent() {
    someNumber = someNumber + 1;

    // When comparing new potential max or min values to the current ones,
    // we ensure that they're indeed greater than the initial max, or
    // smaller than the inital min.
    min = Infinity;
    max = -Infinity;

    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    // The delta time is the difference between now and the last frame, where
    // at the very beginning, right after construction of the element, the
    // last frame is the now from then.
    const delta = now - lastFrameTimeStamp;
    // Overwrite the last frame timestamp with the current one.
    lastFrameTimeStamp = now;
    // Calculate the FPS from the delta. This is done by taking the reciprocal
    // of the delta, which is the number of frames per millisecond, and then
    // multiplying by 1000 to get the number of frames per second.
    fps = (1 / delta) * 1000;

    // Save only the latest 100 timings. We push the new timing onto the end
    // of the array, and then shift off the first element if the array is
    // longer than 100 elements.
    frames.push(fps);
    if (frames.length > 100) {
      frames.shift();
    }

    let sum = 0;
    // For each frame in the stack, we check if it is the new min or max,
    // might replace the old value with the new minimum or maximum, and add
    // it to the sum.
    for (let i = 0; i < frames.length; i++) {
      sum += frames[i];
      min = Math.min(frames[i], min);
      max = Math.max(frames[i], max);
    }
    // Calculate the mean FPS.
    mean = sum / frames.length;
  }
</script>

<p
  class="{$showFps
    ? ''
    : 'hidden'} font-mono uppercase text-xs text-slate-700 tracking-widest absolute ml-2 px-4 py-3 mt-2 sm:mt-2 sm:ml-2 bg-gray-50 bg-opacity-75 backdrop-filter backdrop-blur-sm border border-gray-100 rounded-lg shadow-lg divide-y divide-gray-100 focus:outline-none left-0 top-0"
>
  Latest: {Math.round(fps)} <br />
  AVG of last 100 = {frames.length < 100 ? NaN : Math.round(mean)} <br />
  MIN of last 100 = {frames.length < 100 ? NaN : Math.round(min)} <br />
  MAX of last 100 = {frames.length < 100 ? NaN : Math.round(max)}
</p>
