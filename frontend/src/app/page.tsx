import HeaderBar from "@/components/HeaderBar";
import { Space, Input, Stack, Container, AppShell, AppShellHeader, AppShellMain, Divider } from "@mantine/core";
import AddMessageForm from "@/components/messages/AddMessageForm";
import MessageDisplay from "@/components/messages/MessageDisplay";
export default function Home() {
  return (
    <AppShell header={{ height: "80" }}>
      <AppShellHeader>
        <Container px={20} py={20}>
          <HeaderBar/>
        </Container>
      </AppShellHeader>
      <Space h="xl"/>
      <AppShellMain>
        <Container>
          <Stack>
            <h1 className="text-center">
                <span className="text-5xl">Hey, it's</span>
                <span className="font-bold text-6xl"> nioon<sub className="font-normal text-sm">0.1.6</sub></span>
            </h1>
            <Divider my="xl"/>
            <h1 className="text-center">
                <span className="text-3xl">Leave a</span>
                <span className="font-bold text-4xl"> message</span>
                <span className="text-3xl"> to the world...</span>
            </h1>
            <AddMessageForm/>
            <Divider my="xl"/>
            <MessageDisplay />
          </Stack>
        </Container>
      </AppShellMain>
    </AppShell>
  );
}
