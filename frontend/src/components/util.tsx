import {
  FocusEventHandler,
  MouseEventHandler,
  ReactNode,
  forwardRef,
  useEffect,
  useState,
} from "react";
import { Button } from "../ui/button";
import {
  Check,
  CheckCircle,
  Copy,
  Loader2,
  LogOut,
  Settings,
  User,
} from "lucide-react";
import { Input } from "../ui/input";
import {
  Dialog,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogContent,
  DialogFooter,
} from "@ui/dialog";
import { toast, useToast } from "@ui/use-toast";
import { cn } from "@lib/utils";
import { Link } from "react-router-dom";
import { AUTH_TOKEN_STORAGE_KEY } from "@main";
import { Textarea } from "@ui/textarea";
import { Card } from "@ui/card";

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

export const ActionButton = forwardRef<
  HTMLButtonElement,
  {
    variant?:
      | "link"
      | "default"
      | "destructive"
      | "outline"
      | "secondary"
      | "ghost"
      | null
      | undefined;
    size?: "default" | "sm" | "lg" | "icon" | null | undefined;
    title: string;
    icon: ReactNode;
    disabled?: boolean;
    className?: string;
    onClick?: MouseEventHandler<HTMLButtonElement>;
    onBlur?: FocusEventHandler<HTMLButtonElement>;
    loading?: boolean;
  }
>(
  (
    {
      variant,
      size,
      title,
      icon,
      disabled,
      className,
      loading,
      onClick,
      onBlur,
    },
    ref
  ) => (
    <Button
      size={size}
      variant={variant || "secondary"}
      className={cn("flex items-center justify-between w-[170px]", className)}
      onClick={onClick}
      onBlur={onBlur}
      disabled={disabled}
      ref={ref}
    >
      {title} {loading ? <Loader2 className="w-4 h-4 animate-spin" /> : icon}
    </Button>
  )
);

export const ActionWithDialog = ({
  name,
  title,
  icon,
  disabled,
  loading,
  onClick,
  additional,
  targetClassName,
  variant,
}: {
  name: string;
  title: string;
  icon: ReactNode;
  disabled?: boolean;
  loading?: boolean;
  onClick?: () => void;
  additional?: ReactNode;
  targetClassName?: string;
  variant?:
    | "link"
    | "default"
    | "destructive"
    | "outline"
    | "secondary"
    | "ghost"
    | null
    | undefined;
}) => {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  return (
    <Dialog
      open={open}
      onOpenChange={(open) => {
        setOpen(open);
        setInput("");
      }}
    >
      <DialogTrigger asChild>
        <ActionButton
          className={targetClassName}
          title={title}
          icon={icon}
          disabled={disabled}
          onClick={() => setOpen(true)}
          loading={loading}
          variant={variant}
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
          {additional}
        </div>
        <DialogFooter>
          <ConfirmButton
            title={title}
            icon={icon}
            disabled={disabled || name !== input}
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
  variant,
  size,
  title,
  icon,
  disabled,
  loading,
  onClick,
}: {
  variant?:
    | "link"
    | "default"
    | "destructive"
    | "outline"
    | "secondary"
    | "ghost"
    | null
    | undefined;
  size?: "default" | "sm" | "lg" | "icon" | null | undefined;
  title: string;
  icon: ReactNode;
  onClick?: MouseEventHandler<HTMLButtonElement>;
  loading?: boolean;
  disabled?: boolean;
}) => {
  const [confirmed, set] = useState(false);

  return (
    <ActionButton
      variant={variant}
      size={size}
      title={confirmed ? "Confirm" : title}
      icon={confirmed ? <Check className="w-4 h-4" /> : icon}
      disabled={disabled}
      onClick={
        confirmed
          ? (e) => {
              e.stopPropagation();
              onClick && onClick(e);
              set(false);
            }
          : (e) => {
              e.stopPropagation();
              set(true);
            }
      }
      onBlur={() => set(false)}
      loading={loading}
    />
  );
};

export const Logout = () => (
  <Button
    variant="ghost"
    size="icon"
    onClick={() => {
      localStorage.removeItem(AUTH_TOKEN_STORAGE_KEY);
      location.reload();
    }}
  >
    <LogOut className="w-4 h-4" />
  </Button>
);

export const UserSettings = () => (
  <Link to="/settings">
    <Button variant="ghost" size="icon">
      <Settings className="w-4 h-4" />
    </Button>
  </Link>
);

export const CopyButton = ({
  content,
  className,
}: {
  content: string | undefined;
  className?: string;
}) => {
  const { toast } = useToast();
  const [copied, set] = useState(false);

  useEffect(() => {
    if (copied) {
      toast({ title: "Copied selection" });
      const timeout = setTimeout(() => set(false), 3000);
      return () => {
        clearTimeout(timeout);
      };
    }
  }, [content, copied, toast]);

  return (
    <Button
      className={cn("shrink-0", className)}
      size="icon"
      variant="outline"
      onClick={() => {
        if (!content) return;
        navigator.clipboard.writeText(content);
        set(true);
      }}
      disabled={!content}
    >
      {copied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
    </Button>
  );
};

export const TextUpdateMenu = ({
  title,
  value = "",
  triggerClassName,
  onUpdate,
  placeholder,
  confirmButton,
}: {
  title: string;
  value: string | undefined;
  onUpdate: (value: string) => void;
  triggerClassName?: string;
  placeholder?: string;
  confirmButton?: boolean;
}) => {
  const [open, setOpen] = useState(false);
  const [_value, setValue] = useState(value);
  useEffect(() => setValue(value), [value]);
  const onClick = () => {
    onUpdate(_value);
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
          <div
            className={cn(
              "text-sm text-nowrap overflow-hidden overflow-ellipsis",
              !value && "text-muted-foreground",
              triggerClassName
            )}
          >
            {value || placeholder}
          </div>
        </Card>
      </DialogTrigger>
      <DialogContent className="min-w-[50vw]">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <Textarea
          value={_value}
          onChange={(e) => setValue(e.target.value)}
          placeholder={placeholder}
          onKeyDown={(e) => {
            if (e.key === "Enter") onClick();
          }}
          className="min-h-[200px]"
        />
        <DialogFooter>
          {confirmButton ? (
            <ConfirmButton
              title="Update"
              icon={<CheckCircle className="w-4 h-4" />}
              onClick={onClick}
            />
          ) : (
            <Button
              variant="secondary"
              onClick={onClick}
              className="flex items-center gap-2"
            >
              <CheckCircle className="w-4 h-4" />
              Update
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export const UserAvatar = ({
  avatar,
  size = 4,
}: {
  avatar: string | undefined;
  size?: number;
}) =>
  avatar ? (
    <img src={avatar} alt="Avatar" className={`w-${size}`} />
  ) : (
    <User className={`w-${size}`} />
  );