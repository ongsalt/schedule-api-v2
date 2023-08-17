export function lazy<T extends object>(resolve: () => T) {
    const data: Partial<T> = {};

    const proxy = new Proxy(data, {
        get(target, prop, receiver) {
            return Reflect.get(target, prop, receiver)
        },
        set(target, prop, newValue, receiver) {
            return Reflect.set(target, prop, newValue, receiver)
        }
    })
}