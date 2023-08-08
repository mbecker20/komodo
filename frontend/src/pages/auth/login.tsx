import { Button } from "@ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { useInvalidate } from "@hooks";
import { useEffect, useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { client } from "@main";
import { ThemeToggle } from "@components/util";

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
  const { mutate, isLoading } = useLogin(creds);

  return (
    <div className="flex flex-col min-h-screen">
      <div className="container flex justify-end items-center h-16">
        <ThemeToggle />
      </div>
      <div className="flex justify-center items-center container mt-32">
        <Card className="w-full max-w-[500px] place-self-center">
          <CardHeader>
            <CardTitle>Monitor</CardTitle>
            <CardDescription>Log In</CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <div>
              <Label htmlFor="username">Username</Label>
              <Input
                id="username"
                value={creds.username}
                onChange={({ target }) =>
                  set((c) => ({ ...c, username: target.value }))
                }
              />
            </div>
            <div>
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
            <div className="flex w-full justify-end">
              <Button
                variant="outline"
                onClick={() => mutate(creds)}
                disabled={isLoading}
              >
                Login
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};
