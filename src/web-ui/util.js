export function args(func, ...args) {
  return func.bind(null, ...args)
}
