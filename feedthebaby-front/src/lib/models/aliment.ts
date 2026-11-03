export interface AlimentInDTO {
    id:number;
    name: string;
    quantity: number;
    expire_at: string;
    aliment_type: string;
}

export interface AlimentOutDTO {
    id:number;
    name: string;
    quantity: number;
    expire_at: string;
    aliment_type: string;
    persistence_type: string;
}
