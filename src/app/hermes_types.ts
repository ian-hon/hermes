// export class Channel {
//     public name: string;
//     public id: number;

//     public constructor(name: string, id: number) {
//         this.name = name;
//         this.id = id;
//     }

//     public from_json(s: ): Channel {
//         return new Channel(
//             s
//         );
//     }
// }

export interface Channel {
    id: number;

    name: string;
    description: string;
}

export interface MessageSpecies {
    // UserParticipation(String, bool), // X joined, left; true -> joined
    // Typical(SentMessage),
    // Deletion(i32),
    // Edit(i32, String)
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
