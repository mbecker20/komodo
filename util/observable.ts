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

export function createUserObservable<User = any, Message = any>() {
  const subscribers = new Set<{ user: User; cb: (message: string) => void }>();
  return {
    subscribe: (user: User, callback: (message: string) => void) => {
      const observer = { user, cb: callback };
      subscribers.add(observer);
      return () => {
        subscribers.delete(observer);
      };
    },
    publish: (msg: Message, userFilter?: (user: User) => boolean) => {
      if (userFilter) {
        subscribers.forEach((observer) => {if (userFilter(observer.user)) observer.cb(JSON.stringify(msg))});
      } else {
        subscribers.forEach((observer) => observer.cb(JSON.stringify(msg)));
      }
    },
  };
}