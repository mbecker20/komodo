export function createObservable<Message>() {
  const subscribers = new Set<(message: string) => void>();
  return {
    subscribe: (callback: (message: string) => void) => {
      subscribers.add(callback);
      return () => {
        subscribers.delete(callback);
      };
    },
    publish: (msg: Message) => {
      subscribers.forEach((cb) => cb(JSON.stringify(msg)));
    },
  };
}