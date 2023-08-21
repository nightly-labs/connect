export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

export interface Database {
  public: {
    Tables: {
      tickets: {
        Row: {
          address: string
          created_at: string
          tickets: Json | null
        }
        Insert: {
          address: string
          created_at?: string
          tickets?: Json | null
        }
        Update: {
          address?: string
          created_at?: string
          tickets?: Json | null
        }
        Relationships: []
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}
