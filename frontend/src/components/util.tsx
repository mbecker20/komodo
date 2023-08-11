import { ReactNode, forwardRef, useEffect, useState } from "react";
import { Button, ButtonProps } from "../ui/button";
import { Check, Moon, SunMedium } from "lucide-react";
import { Input } from "../ui/input";
import {
  Dialog,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogContent,
  DialogFooter,
} from "@ui/dialog";
// import { useNavigate } from "react-router-dom";
import { toast } from "@ui/toast/use-toast";
import { cn } from "@util/helpers";

export const WithLoading = ({
  children,
  isLoading,
  loading,
  isError,
  error,
}: {
  children: ReactNode;
  isLoading: boolean;
  loading?: ReactNode;
  isError: boolean;
  error?: ReactNode;
}) => {
  if (isLoading) return <>{loading ?? "loading"}</>;
  if (isError) return <>{error ?? null}</>;
  return <>{children}</>;
};

export const ConfigInput = ({
  placeholder,
  value,
  onChange,
}: {
  placeholder: string;
  value: string | undefined;
  onChange: (s: string) => void;
}) => (
  <Input
    placeholder={placeholder}
    className="max-w-[500px]"
    value={value}
    onChange={({ target }) => onChange(target.value)}
  />
);

export const ThemeToggle = () => {
  const [theme, set] = useState(localStorage.getItem("theme"));

  useEffect(() => {
    localStorage.setItem("theme", theme ?? "dark");
    if (theme === "dark") document.body.classList.remove("dark");
    else document.body.classList.add("dark");
  }, [theme]);

  return (
    <Button
      variant="ghost"
      onClick={() => set(theme === "dark" ? "light" : "dark")}
    >
      <SunMedium className="w-4 h-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      <Moon className="w-4 h-4 absolute rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
    </Button>
  );
};

export const ActionButton = forwardRef<
  HTMLButtonElement,
  {
    title: string;
    icon: ReactNode;
    intent?: ButtonProps["intent"];
    disabled?: boolean;
    className?: string;
    onClick?: () => void;
  }
>(({ title, icon, intent, disabled, className, onClick }, ref) => (
  <Button
    variant="outline"
    className={cn("flex items-center justify-between w-[150px]", className)}
    intent={intent}
    onClick={onClick}
    disabled={disabled}
    ref={ref}
  >
    {title} {icon}
  </Button>
));

export const ActionWithDialog = ({
  name,
  title,
  icon,
  intent,
  disabled,
  onClick,
}: {
  name: string;
  title: string;
  icon: ReactNode;
  intent?: ButtonProps["intent"];
  disabled?: boolean;
  onClick?: () => void;
}) => {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <ActionButton
          title={title}
          icon={icon}
          intent={intent}
          disabled={disabled}
          onClick={() => setOpen(true)}
        />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm {title}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 my-4">
          <p
            onClick={() => {
              navigator.clipboard.writeText(name);
              toast({ title: `Copied "${name}" to clipboard!` });
            }}
            className="cursor-pointer"
          >
            Please enter <b>{name}</b> below to confirm this action.
            <br />
            <span className="text-xs text-muted-foreground">
              You may click the name in bold to copy it
            </span>
          </p>
          <Input value={input} onChange={(e) => setInput(e.target.value)} />
        </div>
        <DialogFooter>
          <ActionButton
            title={title}
            icon={icon}
            intent={intent}
            disabled={name !== input}
            onClick={() => {
              onClick && onClick();
              setOpen(false);
            }}
          />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export const ConfirmButton = ({
  title,
  icon,
  intent,
  disabled,
  onClick,
}: {
  title: string;
  icon: ReactNode;
  intent?: ButtonProps["intent"];
  disabled?: boolean;
  onClick?: () => void;
}) => {
  const [confirmed, set] = useState(false);

  return (
    <>
      <ActionButton
        title={confirmed ? "Confirm" : title}
        icon={confirmed ? <Check className="w-4 h-4" /> : icon}
        intent={intent}
        disabled={disabled}
        onClick={confirmed ? onClick : () => set(true)}
        className={confirmed ? "z-50" : ""}
      />
      {confirmed && (
        <div
          className="absolute z-40 top-0 left-0 w-[100vw] h-[100vh]"
          onClick={() => set(false)}
        />
      )}
    </>
  );
};
