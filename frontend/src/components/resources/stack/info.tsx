import { Section } from "@components/layouts"
import { ReactNode } from "react";
import { useStack } from ".";

export const StackInfo = ({ id, titleOther }: { id: string; titleOther: ReactNode }) => {
	const stack = useStack(id)
	return (
		<Section titleOther={titleOther}>

		</Section>
	)
}