----------------------------------------------------------------------------------
-- Company:
-- Engineer:
--
-- Create Date: 09/25/2024 12:27:25 PM
-- Design Name:
-- Module Name: alu - Behavioral
-- Project Name:
-- Target Devices: f
-- Tool Versions:
-- Description:
--
-- Dependencies:
--
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
--
----------------------------------------------------------------------------------


library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.std_logic_unsigned.all;
use IEEE.numeric_std.all;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity alu is
    Port (
        A: in unsigned (15 downto 0);
        B: in unsigned (15 downto 0);
        FN: in STD_LOGIC_VECTOR (1 downto 0);
        N: out STD_LOGIC;
        Z: out STD_LOGIC;
        Y: out unsigned (15 downto 0)
    );
end alu;

architecture Behavioral of alu is
    signal tmp_Y_wire : unsigned (15 downto 0);
begin
    with FN select
        tmp_Y_wire <= A-B when "00",
                      B-A when "01",
                      A   when "10",
                      B   when others; -- "11"

    Y <= tmp_Y_wire;
    N <= '1' when tmp_Y_wire(15) = '1' else '0';
    Z <= '1' when tmp_Y_wire = "0000000000000000" else '0';
end Behavioral;
