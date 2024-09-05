import HeaderBar from "@/components/HeaderBar";
import HomePage from "@/pages/home";
import { Space, Input, Stack, Container, AppShell, AppShellHeader, AppShellMain } from "@mantine/core";
import Image from "next/image";

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
        <HomePage/>
      </AppShellMain>
    </AppShell>
  );
}
