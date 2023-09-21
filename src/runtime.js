((globalThis) => {
  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalThis.console = {
    log: (...args) => {
      Deno.core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      Deno.core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
  };
})(globalThis);
