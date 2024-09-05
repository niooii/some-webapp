import AddMessageForm from "@/components/AddMessageForm";
import { Space, Input, Stack, Container, AppShell, AppShellHeader, AppShellMain, Textarea, Divider } from "@mantine/core";
import { useForm } from "@mantine/form";

export default function HomePage() {
    
    return (
        <Container>
            <Stack>
            <h1 className="text-center">
                <span className="text-5xl">Hey, it's</span>
                <span className="font-bold text-6xl"> nioon <sub className="font-normal text-sm">0.1.5</sub></span>
            </h1>
            <Divider my="xl"/>
            <AddMessageForm/>
            </Stack>
        </Container>
    );
}