import { client } from "@main";
import { Button } from "@ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export const Signup = () => {
  const nav = useNavigate();
  const [creds, set] = useState({ username: "", password: "" });

  const signup = async () => {
    const { jwt } = await client.auth({
      type: "CreateLocalUser",
      params: creds,
    });
    localStorage.setItem("auth-token", jwt);
    nav("/");
  };

  return (
    <Card className="w-full max-w-[500px] place-self-center">
      <CardHeader>
        <CardTitle>Sign Up</CardTitle>
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
          <Button variant="outline" onClick={signup}>
            Sign Up
          </Button>
        </div>
      </CardContent>
    </Card>
  );
};
