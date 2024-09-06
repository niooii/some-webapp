'use client'

import { Card, Title, Text } from '@mantine/core';
import axios from 'axios';
import useSWR from 'swr'

interface Message {
    title: string;
    content: string;
}

const fetcher = async (url: string) => {
    const { data } = await axios.get<Message[]>(
        url,
        {
            params: {
                amount: "5"
            }
        }
    );

    return data;
};

export default function MessageDisplay() {
    const { data, error, isLoading } = useSWR('/api/messages', fetcher)

    if (error) {
        console.log(error);
        return <div>failed to load</div>
    }
    if (isLoading) return <div>loading...</div>
    return (
        <>
            <Text c="dimmed">Recent messages:</Text>
            {data?.length === 0 ? (
                <Text c="dimmed" size="sm">It's so lonely...</Text>
            ) : (
                data?.map((message, _i) => (
                    <>
                        <Title order={3}>{message.title}</Title>
                        <Text>{message.content}</Text>
                    </>
                ))
            )}
        </>
    );
}