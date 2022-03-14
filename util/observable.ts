export function createObservable<Message>() {
  const subscribers = new Set<(message: Message) => void>();
  return {
    subscribe: (callback: (message: Message) => void) => {
      subscribers.add(callback);
      return () => {
        subscribers.delete(callback);
      };
    },
    publish: (msg: Message) => {
      subscribers.forEach((cb) => cb(msg));
    },
  };
}