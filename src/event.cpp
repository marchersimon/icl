#include "event.h"

std::string Event::getEventName() {
    if((type & 0xF0) == 0xF0) {
        return "System message";
    }
    switch(type) {
        // MIDI
        case NOTE_OFF: return "Note off"; break;				
        case NOTE_ON: return "Note on"; break;
        case KEY_PRESSURE: return "Polyphonic key pressure"; break;
        case CONTROL_CHANGE: return "Control change"; break;
        case PROGRAM_CHANGE: return "Program change"; break;
        case CHANNEL_PRESSURE: return "Channel pressure"; break;
        case PITCH_WHEEL_CHANGE: return "Pitch wheel change"; break;
        // Meta
        case SEQUENCE_NUMBER: return "Sequence number"; break;
        case TEXT_EVENT: return "Text event"; break;
        case COPYRIGHT: return "Copyright notice"; break;
        case SEQUENCE_NAME: return "Sequence or track name"; break;
        case INSTRUMENT: return "Instrument name"; break;
        case LYRIC: return "Lyric text"; break;
        case MARKER_TEXT: return "Marker text"; break;
        case CUE_POINT: return "Cue point"; break;
        case MIDI_CHANNEL_PREFIX: return "MIDI channel prefix assignment"; break;
        case END_OF_TRACK: return "End of track"; break;
        case TEMPO: return "Tempo setting"; break;
        case SMPTE_OFFSET: return "SMPTE offset"; break;
        case TIME_SIGNATURE: return "Time signature"; break;
        case KEY_SIGNATURE: return "Key signature"; break;
        case SEQUENCER_SPECIFIC: return "Sequencer specific event"; break;
    }
    return "Unknown event type";
}

int Event::getEventLength() {
    switch(type) {
        // MIDI
        case KEY_PRESSURE: return 2; break;
        case CONTROL_CHANGE: return 2; break;
        case PROGRAM_CHANGE: return 1; break;
        case CHANNEL_PRESSURE: return 1; break;
        case PITCH_WHEEL_CHANGE: return 2; break;
        // Meta
        case SEQUENCE_NUMBER: return 2; break;
        case MIDI_CHANNEL_PREFIX: return 1; break;
        case END_OF_TRACK: return 0; break;
        case TEMPO: return 3; break;
        case SMPTE_OFFSET: return 5; break;
        case TIME_SIGNATURE: return 4; break;
        case KEY_SIGNATURE: return 2; break;
    }
    return -1;
}

std::string Event::getNoteName() {
    std::string noteName;
    switch(note % 12) {
        case 0: noteName.append("C"); break;
        case 1: noteName.append("C#"); break;
        case 2: noteName.append("D"); break;
        case 3: noteName.append("D#"); break;
        case 4: noteName.append("E"); break;
        case 5: noteName.append("F"); break;
        case 6: noteName.append("F#"); break;
        case 7: noteName.append("G"); break;
        case 8: noteName.append("G#"); break;
        case 9: noteName.append("A"); break;
        case 10: noteName.append("A#"); break;
        case 11: noteName.append("B"); break;
    }

    noteName += std::to_string((int)(note / 12) - 1);
    return noteName;
}

void Event::stripChannel() {
    type &= 0xF0;
}

int Event::getChannel() {
    return type & 0x0F;
}

void Event::print(int pos, uint8_t *file, int len) {
    std::string row;
    row += formatColumn(Log::to_hex_string(pos), 6);
    row += " | ";
    std::string content;
    for(int i = 0; i < len; i++) {
        content += Log::to_hex_string(file[pos + i], false);
        content += " ";
    }
    if(content.length() > 39) {
        content.replace(27, content.length() - 34, "[...]");
    }
    row += formatColumn(content, 39);
    //row += content;
    row += "| ";
    row += formatColumn(std::to_string(totalTime), 6);
    row += " | ";
    row += formatColumn(std::to_string(delta), 6);
    row += " | ";
    row += formatColumn(getEventName(), 25);
    row += " | ";
    if(meta) {
        row += "          ";
    } else {
        row += formatColumn("Channel " + std::to_string(getChannel()), 10);
    }

    row += " | ";
    if(type == NOTE_ON || type == NOTE_OFF) {
        row += formatColumn("Note " + getNoteName(), 9);
        row += "at velocity " + std::to_string(velocity);
    } else if(type == TEMPO) {
        row += formatColumn(std::to_string(tempo), 6);
        row += " us per quarter note";
    }

    Log::debug(row);
}

std::string Event::formatColumn(std::string s, int width) {
    while(width - (int)s.length() > 0) {
        s += " ";
    }
    return s;
}