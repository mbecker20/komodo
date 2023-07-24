import { Button } from "@ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { useLogin } from "@hooks";
import { useState } from "react";

export const Login = () => {
  const { mutate, isLoading } = useLogin();
  const [creds, set] = useState({ username: "", password: "" });

  return (
    <Card className="w-full max-w-[500px] place-self-center">
      <CardHeader>
        <CardTitle>Log In</CardTitle>
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
  );
};
