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
    name: string;
    id: number;
}