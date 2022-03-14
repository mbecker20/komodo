export type User = {
  _id: string;
  username: string;
  permissions: number;
  password?: string;
  githubID?: string;
  avatar?: string;
};