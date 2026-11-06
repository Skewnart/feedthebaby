export interface AlimentInDTO {
    id:number;
    name: string;
    quantity: number;
    expire_at: string;
    aliment_type: string;
}

export interface AlimentInNewIdDTO {
    old_id: number;
    new_id: number;
}

export interface AlimentOutDTO {
    id:number;
    name: string;
    quantity: number;
    expire_at: string;
    aliment_type: string;
    persistence_type: string;
}
