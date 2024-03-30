import { UserX } from "lucide-react";

export const UserDisabled = () => {
  return (
    <div className="w-full h-screen flex justify-center items-center">
      <div className="flex flex-col gap-4 justify-center items-center">
        <UserX className="w-16 h-16" />
        User Not Enabled
      </div>
    </div>
  );
};
