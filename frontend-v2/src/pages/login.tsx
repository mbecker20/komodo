import { Button } from "@ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { useInvalidate } from "@lib/hooks";
import { useEffect, useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { client } from "@main";
import { ThemeToggle } from "@ui/theme";

export const Signup = ({ setSignup }: { setSignup: (f: false) => void }) => {
  // const { refetch } = useRead("GetUser", {});
  const [creds, set] = useState({ username: "", password: "" });
  const { mutateAsync, isLoading } = useMutation(
    (creds: { username: string; password: string }) =>
      client.auth({
        type: "CreateLocalUser",
        params: creds,
      })
  );

  const signup = async () => {
    const { jwt } = await mutateAsync(creds);
    localStorage.setItem("monitor-auth-token", jwt);
    location.reload();
  };

  return (
    <div className="flex flex-col min-h-screen">
      <div className="container flex justify-end items-center h-16">
        <ThemeToggle />
      </div>
      <div className="flex justify-center items-center container mt-32">
        <Card className="w-full max-w-[500px] place-self-center">
          <CardHeader className="flex-col">
            <CardTitle className="text-xl">Monitor</CardTitle>
            <CardDescription>Sign Up</CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <div className="flex flex-col gap-2">
              <Label htmlFor="username">Username</Label>
              <Input
                id="username"
                value={creds.username}
                onChange={({ target }) =>
                  set((c) => ({ ...c, username: target.value }))
                }
              />
            </div>
            <div className="flex flex-col gap-2">
              <Label htmlFor="password">Password</Label>
              <Input
                id="password"
                type="password"
                value={creds.password}
                onChange={({ target }) =>
                  set((c) => ({ ...c, password: target.value }))
                }
              />
            </div>
          </CardContent>
          <CardFooter className="flex gap-4 w-full justify-end">
            <Button
              onClick={() => setSignup(false)}
              disabled={isLoading}
              variant="outline"
            >
              Log In
            </Button>
            <Button onClick={signup} disabled={isLoading}>
              Sign Up
            </Button>
          </CardFooter>
        </Card>
      </div>
    </div>
  );
};

type LoginCredentials = { username: string; password: string };

const useLogin = (creds: LoginCredentials) => {
  const invalidate = useInvalidate();

  const mutation = useMutation(
    (creds: LoginCredentials) => client.login(creds),
    {
      onSuccess: (jwt) => {
        localStorage.setItem("monitor-auth-token", jwt ?? "");
        invalidate(["GetUser"]);
      },
    }
  );

  useEffect(() => {
    const handler = (e: KeyboardEvent) =>
      e.key === "Enter" && !mutation.isLoading && mutation.mutate(creds);
    addEventListener("keydown", handler);
    return () => {
      removeEventListener("keydown", handler);
    };
  });

  return mutation;
};

export const Login = () => {
  const [creds, set] = useState({ username: "", password: "" });
  const [signup, setSignup] = useState(false);
  const { mutate, isLoading } = useLogin(creds);

  if (signup) return <Signup setSignup={setSignup} />;

  return (
    <div className="flex flex-col min-h-screen">
      <div className="container flex justify-end items-center h-16">
        <ThemeToggle />
      </div>
      <div className="flex justify-center items-center container mt-32">
        <Card className="w-full max-w-[500px] place-self-center">
          <CardHeader className="flex-col">
            <CardTitle className="text-xl">Monitor</CardTitle>
            <CardDescription>Log In</CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <div className="flex flex-col gap-2">
              <Label htmlFor="username">Username</Label>
              <Input
                id="username"
                value={creds.username}
                onChange={({ target }) =>
                  set((c) => ({ ...c, username: target.value }))
                }
              />
            </div>
            <div className="flex flex-col gap-2">
              <Label htmlFor="password">Password</Label>
              <Input
                id="password"
                type="password"
                value={creds.password}
                onChange={({ target }) =>
                  set((c) => ({ ...c, password: target.value }))
                }
              />
            </div>
          </CardContent>
          <CardFooter className="flex gap-4 w-full justify-end">
            <Button
              onClick={() => setSignup(true)}
              disabled={isLoading}
              variant="outline"
            >
              Signup
            </Button>
            <Button onClick={() => mutate(creds)} disabled={isLoading}>
              Login
            </Button>
          </CardFooter>
        </Card>
      </div>
    </div>
  );
};
