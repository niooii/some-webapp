'use client'
import { Button, Checkbox, Group, Space, Textarea, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";

export default function AddMessageForm() {
    const form = useForm({
            mode: 'uncontrolled',
            initialValues: {
                content: '',
                title: '',
            },
        
            validate: {
                content: (value) => value.length != 0 ? null : 'Content is required',
                title: (value) => value.length != 0 ? null : 'Title is required'
            },
    });

    return (
    <form onSubmit={form.onSubmit((values) => console.log(values))}>
        <TextInput
            withAsterisk
            label="Title"
            placeholder="Title"
            key={form.key('title')}
            {...form.getInputProps('title')}
        />

        <Space py={10}/>

        <Textarea
            withAsterisk
            label="Content"
            placeholder="your@email.com"
            key={form.key('content')}
            maxLength={2000}
            maxRows={5}
            autosize={true}
            {...form.getInputProps('content')}
        />

        <Space py={10}/>

        <Button type="submit">Submit</Button>
    </form>
    );
}