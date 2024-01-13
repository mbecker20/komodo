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
import { useAuth, useInvalidate } from "@lib/hooks";
import { useEffect, useState } from "react";
import { ThemeToggle } from "@ui/theme";

export const Signup = ({ setSignup }: { setSignup: (f: false) => void }) => {
  const [creds, set] = useState({ username: "", password: "" });
  const { mutateAsync, isPending } = useAuth("CreateLocalUser");

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
              disabled={isPending}
              variant="outline"
            >
              Log In
            </Button>
            <Button onClick={signup} disabled={isPending}>
              Sign Up
            </Button>
          </CardFooter>
        </Card>
      </div>
    </div>
  );
};

export const Login = () => {
  const [creds, set] = useState({ username: "", password: "" });
  const [signup, setSignup] = useState(false);

  const inv = useInvalidate();
  const { mutate, isPending } = useAuth("LoginLocalUser", {
    onSuccess: ({ jwt }) => {
      localStorage.setItem("monitor-auth-token", jwt);
      inv(["GetUser"]);
    },
  });

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Enter" && !isPending) mutate(creds);
    };
    addEventListener("keydown", handler);
    return () => {
      removeEventListener("keydown", handler);
    };
  });

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
              disabled={isPending}
              variant="outline"
            >
              Signup
            </Button>
            <Button onClick={() => mutate(creds)} disabled={isPending}>
              Login
            </Button>
          </CardFooter>
        </Card>
      </div>
    </div>
  );
};
