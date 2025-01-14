
export type BookmarkedNumberEntry = {
  entry_id: string

  owner_addr: string
  number: number
  memo_text: string
  marked_as_public_at_in_ms: null | number,

  created_at_in_ms: number
  updated_at_in_ms: number
}
