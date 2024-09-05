'use client'
import { Stack, Container, Group, Burger, Button } from '@mantine/core';

export default function HeaderBar() {
  return (
    <header>
      <Group justify="center" gap="xl" grow>
        <Button variant="default">Github</Button>
        <Button variant="default">Second</Button>
        <Button variant="default">Third</Button>
      </Group>  
    </header>
  );
}