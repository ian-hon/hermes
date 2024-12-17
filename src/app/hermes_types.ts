export interface Channel {
    id: number;

    name: string;
    description: string;

    invite: number;
}

export class Message {
    public id: number;
    public author: string;
    public content: string;
    public timestamp: number;
    public edited_timestamp: number;
    public reply: number | null;
    public image: string | null;

    public constructor(id: number, author: string, content: string, timestamp: number, edited_timestamp: number, reply: number | null, image : string | null) {
        this.id = id;
        this.author = author;
        this.content = content;
        this.timestamp = timestamp;
        this.edited_timestamp = edited_timestamp;
        this.reply = reply;
        this.image = image;
    }
}
