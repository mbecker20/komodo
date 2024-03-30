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
  Box,
  Boxes,
  Check,
  Copy,
  FolderTree,
  Key,
  Loader2,
  LogOut,
  Moon,
  Settings,
  SunMedium,
  Tag,
  User,
  UserCircle2,
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
import { RESOURCE_TARGETS, cn } from "@lib/utils";
import {
  useInvalidate,
  useRead,
  useResourceParamType,
  useWrite,
} from "@lib/hooks";
import { Link, useNavigate, useParams } from "react-router-dom";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { ResourceComponents } from "./resources";
import { Types } from "@monitor/client";
import { AUTH_TOKEN_STORAGE_KEY } from "@main";

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
      variant={variant || "outline"}
      className={cn("flex items-center justify-between w-[150px]", className)}
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
}: {
  name: string;
  title: string;
  icon: ReactNode;
  disabled?: boolean;
  loading?: boolean;
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
          disabled={disabled}
          onClick={() => setOpen(true)}
          loading={loading}
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

export const CopyResource = ({
  id,
  disabled,
  type,
}: {
  id: string;
  disabled?: boolean;
  type: "Deployment" | "Build";
}) => {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");

  const nav = useNavigate();
  const inv = useInvalidate();
  const { mutate } = useWrite(`Copy${type}`, {
    onSuccess: (res) => {
      inv([`List${type}s`]);
      nav(`/${type.toLowerCase()}s/${res._id?.$oid}`);
    },
  });

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <ActionButton
          title="Copy"
          icon={<Copy className="w-4 h-4" />}
          disabled={disabled}
          onClick={() => setOpen(true)}
        />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Copy {type}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 my-4">
          <p>Provide a name for the newly created {type.toLowerCase()}.</p>
          <Input value={name} onChange={(e) => setName(e.target.value)} />
        </div>
        <DialogFooter>
          <ActionButton
            title="Confirm"
            icon={<Check className="w-4 h-4" />}
            disabled={!name}
            onClick={() => {
              mutate({ id, name });
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
              onClick && onClick(e);
              set(false);
            }
          : () => set(true)
      }
      onBlur={() => set(false)}
      loading={loading}
    />
  );
};

export const ResourceTypeDropdown = () => {
  const type = useResourceParamType();
  const Components = ResourceComponents[type];

  const [icon, title] = type
    ? [<Components.Icon />, type + "s"]
    : location.pathname === "/tree"
    ? [<FolderTree className="w-4 h-4" />, "Tree"]
    : location.pathname === "/keys"
    ? [<Key className="w-4 h-4" />, "Api Keys"]
    : location.pathname === "/tags"
    ? [<Tag className="w-4 h-4" />, "Tags"]
    : location.pathname === "/users"
    ? [<UserCircle2 className="w-4 h-4" />, "Users"]
    : [<Box className="w-4 h-4" />, "Dashboard"];

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" className="w-36 justify-between px-3">
          <div className="flex items-center gap-2">
            {icon}
            {title}
          </div>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-36" side="bottom">
        <DropdownMenuGroup>
          <Link to="/">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Box className="w-4 h-4" />
              Dashboard
            </DropdownMenuItem>
          </Link>

          <DropdownMenuSeparator />

          <Link to="/resources">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Boxes className="w-4 h-4" />
              Resources
            </DropdownMenuItem>
          </Link>
          <Link to="/tree">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <FolderTree className="w-4 h-4" />
              Tree
            </DropdownMenuItem>
          </Link>

          <DropdownMenuSeparator />

          {RESOURCE_TARGETS.map((rt) => {
            const RTIcon = ResourceComponents[rt].Icon;
            return (
              <Link key={rt} to={`/${rt.toLowerCase()}s`}>
                <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
                  <RTIcon />
                  {rt}s
                </DropdownMenuItem>
              </Link>
            );
          })}

          <DropdownMenuSeparator />

          <Link to="/tags">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Tag className="w-4 h-4" />
              Tags
            </DropdownMenuItem>
          </Link>

          <DropdownMenuSeparator />

          <Link to="/keys">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Box className="w-4 h-4" />
              Api Keys
            </DropdownMenuItem>
          </Link>
          <Link to="/users">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <UserCircle2 className="w-4 h-4" />
              Users
            </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

export const ResourcesDropdown = () => {
  const type = useResourceParamType();
  const id = useParams().id as string;
  const list = useRead(`List${type}s`, {}).data;

  const selected = list?.find((i) => i.id === id);
  const Components = ResourceComponents[type];

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" className="w-48 justify-between px-3">
          <div className="flex items-center gap-2">
            <Components.Icon id={selected?.id} />
            {selected ? selected.name : `All ${type}s`}
          </div>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-48" side="bottom">
        <DropdownMenuGroup>
          <Link to={`/${type.toLowerCase()}s`}>
            <DropdownMenuItem className="flex items-center gap-2">
              <Components.Icon />
              All {type}s
            </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>
        <DropdownMenuGroup>
          {!list?.length && (
            <DropdownMenuItem disabled>No {type}s Found.</DropdownMenuItem>
          )}

          {list?.map(({ id, name }) => (
            <Link key={id} to={`/${type.toLowerCase()}s/${id}`}>
              <DropdownMenuItem className="flex items-center gap-2">
                <Components.Icon id={id} />
                {name}
              </DropdownMenuItem>
            </Link>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
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

export const UserDropdown = () => {
  // const user = useRead("GetUser", {}).data;
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon">
          <User className="w-4 h-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <Logout />
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

export const CopyButton = ({ content }: { content: string | undefined }) => {
  const { toast } = useToast();
  const [copied, set] = useState(false);

  useEffect(() => {
    if (copied) {
      toast({ title: `Copied "${content}"` });
      const timeout = setTimeout(() => set(false), 3000);
      return () => {
        clearTimeout(timeout);
      };
    }
  }, [content, copied, toast]);

  return (
    <Button
      className="shrink-0"
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

const alert_level_color = (level: Types.SeverityLevel) => {
  if (level === Types.SeverityLevel.Ok) return "green-500";
  if (level === Types.SeverityLevel.Warning) return "orange-500";
  if (level === Types.SeverityLevel.Critical) return "red-500";
};

// const alert_level_fill_color = (level: Types.SeverityLevel) => {
//   return `fill-${alert_level_color(level)}`;
// };

const alert_level_text_color = (level: Types.SeverityLevel) => {
  return `text-${alert_level_color(level)}`;
};

export const AlertLevel = ({ level }: { level: Types.SeverityLevel }) => {
  return <div className={alert_level_text_color(level)}>{level}</div>;
};
