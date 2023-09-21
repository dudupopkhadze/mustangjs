((globalContext) => {
  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalContext.console = {
    log: (...args) => {
      Deno.core.print(`[log]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      Deno.core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
    warn: (...args) => {
      Deno.core.print(`[wrn]: ${argsToMessage(...args)}\n`, true);
    },
  };
})(globalThis);
