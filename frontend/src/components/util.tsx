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
  Check,
  CheckCircle,
  ChevronDown,
  ChevronUp,
  Copy,
  Database,
  HardDrive,
  Loader2,
  LogOut,
  Network,
  Settings,
  Tags,
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
import { snake_case_to_upper_space_case } from "@lib/formatting";
import {
  ColorIntention,
  hex_color_by_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { Types } from "@monitor/client";
import { Badge } from "@ui/badge";
import { Section } from "./layouts";

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
      className={cn("flex items-center justify-between w-[190px]", className)}
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
  className,
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
  className?: string;
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
      className={className}
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
  titleRight,
  value = "",
  triggerClassName,
  onUpdate,
  placeholder,
  confirmButton,
  disabled,
  fullWidth,
  open,
  setOpen,
  triggerHidden,
}: {
  title: string;
  titleRight?: ReactNode;
  value: string | undefined;
  onUpdate: (value: string) => void;
  triggerClassName?: string;
  placeholder?: string;
  confirmButton?: boolean;
  disabled?: boolean;
  fullWidth?: boolean;
  open?: boolean;
  setOpen?: (open: boolean) => void;
  triggerHidden?: boolean;
}) => {
  const [_open, _setOpen] = useState(false);
  const [__open, __setOpen] = [open ?? _open, setOpen ?? _setOpen];
  const [_value, setValue] = useState(value);
  useEffect(() => setValue(value), [value]);
  const onClick = () => {
    onUpdate(_value);
    __setOpen(false);
  };

  return (
    <Dialog open={__open} onOpenChange={__setOpen}>
      <DialogTrigger asChild>
        <Card
          className={cn(
            "px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer",
            fullWidth ? "w-full" : "w-fit",
            triggerHidden && "hidden"
          )}
        >
          <div
            className={cn(
              "text-sm text-nowrap overflow-hidden overflow-ellipsis",
              (!value || !!disabled) && "text-muted-foreground",
              triggerClassName
            )}
          >
            {value || placeholder}
          </div>
        </Card>
      </DialogTrigger>
      <DialogContent className="min-w-[50vw]">
        {titleRight && (
          <div className="flex items-center gap-4">
            <DialogHeader>
              <DialogTitle>{title}</DialogTitle>
            </DialogHeader>
            {titleRight}
          </div>
        )}
        {!titleRight && (
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
          </DialogHeader>
        )}

        <Textarea
          value={_value}
          onChange={(e) => setValue(e.target.value)}
          placeholder={placeholder}
          className="min-h-[200px]"
          disabled={disabled}
        />
        {!disabled && (
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
        )}
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
    <img src={avatar} alt="Avatar" className={`w-${size} h-${size}`} />
  ) : (
    <User className={`w-${size} h-${size}`} />
  );

export const StatusBadge = ({
  text,
  intent,
}: {
  text: string | undefined;
  intent: ColorIntention;
}) => {
  if (!text) return null;

  const color = text_color_class_by_intention(intent);
  const background = hex_color_by_intention(intent) + "25";

  const _text = text === Types.ServerState.NotOk ? "Not Ok" : text;

  return (
    <p
      className={cn(
        "px-2 py-1 w-fit text-xs text-white rounded-md font-medium tracking-wide",
        color
      )}
      style={{ background }}
    >
      {snake_case_to_upper_space_case(_text).toUpperCase()}
    </p>
  );
};

export const DockerOptions = ({
  options,
}: {
  options: Record<string, string> | undefined;
}) => {
  if (!options) return null;
  const entries = Object.entries(options);
  if (entries.length === 0) return null;
  return (
    <div className="flex gap-2 flex-wrap">
      {entries.map(([key, value]) => (
        <Badge key={key} variant="secondary">
          {key} = {value}
        </Badge>
      ))}
    </div>
  );
};

export const DockerLabelsSection = ({
  labels,
}: {
  labels: Record<string, string> | undefined;
}) => {
  if (!labels) return null;
  const entries = Object.entries(labels);
  if (entries.length === 0) return null;
  return (
    <Section title="Labels" icon={<Tags className="w-4 h-4" />}>
      <div className="flex gap-2 flex-wrap">
        {entries.map(([key, value]) => (
          <Badge key={key} variant="secondary" className="flex gap-1">
            <span className="text-muted-foreground">{key}</span>
            <span className="text-muted-foreground">=</span>
            <span
              title={value}
              className="font-extrabold text-nowrap max-w-[200px] overflow-hidden text-ellipsis"
            >
              {value}
            </span>
          </Badge>
        ))}
      </div>
    </Section>
  );
};

export const ShowHideButton = ({
  show,
  setShow,
}: {
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  return (
    <Button
      size="sm"
      variant="outline"
      className="gap-4"
      onClick={() => setShow(!show)}
    >
      {show ? "Hide" : "Show"}
      {show ? <ChevronUp className="w-4" /> : <ChevronDown className="w-4" />}
    </Button>
  );
};

type DockerResourceType = "container" | "network" | "image" | "volume";

const DOCKER_LINK_ICONS: {
  [type in DockerResourceType]: React.FC;
} = {
  container: () => <Box className="w-4 h-4" />,
  network: () => <Network className="w-4 h-4" />,
  image: () => <HardDrive className="w-4 h-4" />,
  volume: () => <Database className="w-4 h-4" />,
};

export const DockerResourceLink = ({
  server_id,
  name,
  type,
  extra,
}: {
  server_id: string;
  name?: string;
  type: "container" | "network" | "image" | "volume";
  extra?: ReactNode;
}) => {
  if (!name) return "Unknown";

  const Icon = DOCKER_LINK_ICONS[type];

  return (
    <Link
      to={`/servers/${server_id}/${type}/${encodeURIComponent(name)}`}
      className="px-0"
    >
      <Button variant="link" className="px-0 gap-2">
        <Icon />
        <div
          title={name}
          className="max-w-[200px] lg:max-w-[300px] overflow-hidden overflow-ellipsis"
        >
          {name}
        </div>
        {extra && <div className="no-underline">{extra}</div>}
      </Button>
    </Link>
  );
};

export const DockerResourcePageName = ({ name: _name }: { name?: string }) => {
  const name = _name ?? "Unknown";
  return (
    <h1
      title={name}
      className="text-3xl max-w-[300px] md:max-w-[500px] xl:max-w-[700px] overflow-hidden overflow-ellipsis"
    >
      {name}
    </h1>
  );
};
