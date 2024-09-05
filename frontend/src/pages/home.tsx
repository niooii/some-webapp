import { Space, Input, Stack, Container, AppShell, AppShellHeader, AppShellMain, Textarea, Divider } from "@mantine/core";

export default function HomePage() {
    return (
        <Container>
            <Stack>
            <h1 className="text-center">
                <span className="text-5xl">Hey, it's</span>
                <span className="font-bold text-6xl"> nioon <sub className="text-sm">0.1.5</sub></span>
                <span className="text-5xl"> ...</span>
            </h1>
            <Divider my="xl"/>
            <Textarea
                label="Message contents"
                variant="filled"    
                radius="md"
                maxRows={16}
                description=" "
                placeholder="Hello World!"
            />
            </Stack>
        </Container>
    );
}