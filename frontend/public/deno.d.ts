interface Deno {
	exit(code: number): void;
}

declare global {
	var Deno: Deno;
}

export {}